use std::error::Error;
use std::fs::{create_dir, File};
use std::io::{Read};
use std::path::{Path, PathBuf};

use crate::util::timer::Timer;

use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result};
use std::fmt;

use serde_yaml;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Config {
    version: String,
    debug: bool,
    pub(crate) first_run: bool,
    db_file: String,
}

#[derive(Debug)]
struct MError(String);


fn load_db(config: &Config) -> Result<Connection, Box<dyn Error>> {
    let data_dir = "data";

    // Create the `data` folder if it doesn't exist
    if !PathBuf::from(data_dir).exists() {
        create_dir(data_dir)?;
        println!("Created data directory");
        if !(config.first_run) {
            warn!("The data directory was not found.
             This is normal if this is the first time you are running the program.");
        }
    }
    // Use the `db_path` variable to open or create the database file
    let conn: Connection = Connection::open(&config.db_file)?;

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

    config.first_run = false;
    let yaml = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, yaml)?;

    Ok(output_config)
}

pub fn load_db_helper() -> Result<Connection, Box<dyn Error>> {
    let config = load_config()?;
    load_db(&config)
}