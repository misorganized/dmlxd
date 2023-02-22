use std::error::Error;
use std::fs::{create_dir, File};
use std::io::Read;
use std::path::{PathBuf};
use crate::util::timer::Timer;
use serde::Deserialize;
use sqlite::Connection;
use std::fmt;


#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    version: String,
    debug: bool,
    first_run: bool,
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

    let conn = match load_db() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error loading database: {}", e);
            return Err(Box::new(MError("An error occurred".to_string())));
        }
    };

    timer.pointer("Loading database");

    let mut timer = Timer::new();

    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return Err(Box::new(MError("An error occurred".to_string())));
        }
    };
    println!("Config: {:?}", config);

    timer.pointer("Loading config");
    total_load_time.pointer("Total load time");

    Ok((conn, config))
}



fn load_db() -> Result<Connection, Box<dyn Error>> {
    let data_dir = "data";
    let db_file = "main.db";
    let db_path = PathBuf::from(data_dir).join(db_file);

    let mut timer = Timer::new();

    // Create the `data` folder if it doesn't exist
    if !PathBuf::from(data_dir).exists() {
        create_dir(data_dir)?;
        println!("Created data directory");
    }

    timer.pointer("Creating data directory");

    let mut timer = Timer::new();
    // Use the `db_path` variable to open or create the database file
    let conn = Connection::open(&db_path)?;

    // Create the `users` table if it doesn't exist
    conn.execute("
        CREATE TABLE IF NOT EXISTS users (
            login INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            public_key TEXT NOT NULL UNIQUE,
            ip_address TEXT NOT NULL,
            port INTEGER NOT NULL
        );
    ")?;

    timer.pointer("Accessing database");

    Ok(conn)
}


fn load_config() -> Result<Config, Box<dyn Error>> {
    // Open the config file
    let mut file = File::open("data/config.yaml")?;

    // Read the contents of the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the YAML into a Config struct
    let config = serde_yaml::from_str(&contents)?;

    Ok(config)
}

