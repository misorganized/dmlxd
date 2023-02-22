use std::error::Error;
use std::fs::{create_dir, File};
use std::io::{Read, stdin, stdout, Write};
use std::path::{Path, PathBuf};

use crate::util::timer::Timer;
use crate::util::user::User;

use serde::{Deserialize, Serialize};
use rusqlite::{Connection, params, Result};
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

pub fn first_login(conn: &Connection) {
    println!("First login");

    print!("Please enter your display name: ");
    stdout().flush().unwrap();
    let mut display_name = String::new();
    stdin().read_line(&mut display_name).unwrap();
    display_name = display_name.trim().to_string();

    print!("{}", display_name);

    let user = User {
        login: display_name,
        name: "Alice".to_string(),
        public_key: "public_key".to_string(),
        ip_address: "127.0.0.1".to_string(),
        port: 1096,
    };


    conn.execute(
        "INSERT INTO users (login, name, public_key, ip_address, port) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![&user.login, &user.name, &user.public_key, &user.ip_address, &user.port],
    ).unwrap();
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

