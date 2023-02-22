use std::fs::{create_dir, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::util::timer::Timer;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    version: String,
    debug: bool,
    first_run: bool,
    db_file: String,
}

pub(crate) fn init() {
    let mut timer = Timer::new();
    load_db();
    timer.pointer("Loading database");

    let mut timer = Timer::new();
    load_config();
    timer.pointer("Loading config");
}

fn load_db() {
    let data_dir = "data";
    let db_file = "main.db";
    let db_path = PathBuf::from(data_dir).join(db_file);

    let mut timer = Timer::new();

    // Create the `data` folder if it doesn't exist
    if !PathBuf::from(data_dir).exists() {
        create_dir(data_dir).expect("Failed to create data directory");
        println!("Created data directory");
    }

    timer.pointer("Creating data directory");

    let mut timer = Timer::new();
    // Use the `db_path` variable to open or create the database file
    let conn = sqlite::Connection::open(&db_path).expect("Failed to open database");


    // Create the `users` table if it doesn't exist
    conn.execute("
        CREATE TABLE IF NOT EXISTS users (
            login INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            public_key TEXT NOT NULL UNIQUE,
            ip_address TEXT NOT NULL,
            port INTEGER NOT NULL
        );
    ").unwrap();

    timer.pointer("Accessing database");
}

fn load_config () {
    let config_path = Path::new("data").join("config.yaml");

    let mut file = match File::open(&config_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening config file: {}", e);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error reading config file: {}", e);
        return;
    }

    let config: Config = match serde_yaml::from_str(&contents) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error deserializing config file: {}", e);
            return;
        }
    };

    println!("Config: {:?}", config);
}