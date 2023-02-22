use std::error::Error;
use std::fs::{create_dir, File};
use std::io::{Read};
use std::path::{Path, PathBuf};

use crate::util::timer::Timer;


use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result};
use std::fmt;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Config {
    version: String,
    debug: bool,
    pub(crate) first_run: bool,
    db_file: String,
}

#[derive(Debug)]
struct MError(String);

impl fmt::Display for MError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MError {}

pub(crate) fn init() -> Result<(Connection, Config), Box<dyn Error>> {
    let mut total_load_time = Timer::new();

    let mut timer = Timer::new();

    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return Err(Box::new(MError("An error occurred".to_string())));
        }
    };

    timer.pointer("Loading config");

    let mut timer = Timer::new();

    let conn = match load_db(&config.db_file) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error loading database: {}", e);
            return Err(Box::new(MError("An error occurred".to_string())));
        }
    };

    timer.pointer("Loading database");
    total_load_time.pointer("Total load time");

    Ok((conn, config))
}

fn load_db(db_path: &String) -> Result<Connection, Box<dyn Error>> {
    let data_dir = "data";

    // Create the `data` folder if it doesn't exist
    if !PathBuf::from(data_dir).exists() {
        create_dir(data_dir)?;
        println!("Created data directory");
    }
    // Use the `db_path` variable to open or create the database file
    let conn: Connection = Connection::open(&db_path)?;

    // Create the `users` table if it doesn't exist
    conn.execute("
        CREATE TABLE IF NOT EXISTS users (
            login TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            public_key TEXT NOT NULL UNIQUE,
            ip_address TEXT NOT NULL,
            port INTEGER NOT NULL
        );
    ", ())?;

    Ok(conn)
}


fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_path = Path::new("data/config.yaml");

    // Open the config file
    let mut file = File::open(&config_path)?;

    // Read the contents of the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the YAML into a Config struct
    let mut config: Config= serde_yaml::from_str(&contents)?;

    let output_config = config.clone();

    config.first_run = true;
    let yaml = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, yaml)?;

    Ok(output_config)
}

