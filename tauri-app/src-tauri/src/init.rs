use std::error::Error;
use std::fs::{create_dir, File};
use std::io::{Read, Write};
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

    let conn = match load_db(&config) {
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

fn load_db(config: &Config) -> Result<Connection, Box<dyn Error>> {
    let data_dir = "data";

    // Create the `data` folder if it doesn't exist
    if !PathBuf::from(data_dir).exists() {
        create_dir(data_dir)?;
        println!("Created data directory");
        if !(config.first_run) {
            log::warn!("The data directory was not found.
             This is normal if this is the first time you are running the program.");
        }
    }
    // Use the `db_path` variable to open or create the database file
    let conn: Connection = Connection::open(&config.db_file)?;

    // Create the `users` table if it doesn't exist
    conn.execute("
        CREATE TABLE IF NOT EXISTS users (
            login TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            name TEXT NOT NULL,
            public_key TEXT NOT NULL UNIQUE,
            ip_address TEXT NOT NULL,
            port INTEGER NOT NULL
        );
    ", ())?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS contacts (
            login TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            public_key TEXT NOT NULL UNIQUE,
            ip_address TEXT NOT NULL,
            port INTEGER NOT NULL
        );
    ", ())?;

    Ok(conn)
}

pub fn db_conn() -> Connection {
    let config = load_config().unwrap();
    let conn: Connection = Connection::open(&config.db_file).unwrap();
    return conn;
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

    Ok(output_config)
}

pub fn write_first_login() {
    let mut config = load_config().unwrap();
    config.first_run = false;
    let config_path = Path::new("data/config.yaml");
    let mut file = File::create(&config_path).unwrap();
    let config_string = serde_yaml::to_string(&config).unwrap();
    file.write_all(config_string.as_bytes()).unwrap();
}

#[tauri::command]
pub fn is_first_login() -> bool {
    let config = load_config().unwrap();
    println!("First run: {}", config.first_run);
    return config.first_run;
}
