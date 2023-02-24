mod init;

use rusqlite::params;
use crate::init::init;
use crate::util::register::generate_keypair;
use crate::util::user::{update_user, User};

mod util {
    pub mod timer;
    pub mod user;
    pub mod register;
}

mod comms {
    pub mod init_comm;
}

#[tauri::command]
async fn get_user_data() -> Result<User, String> {
    let (conn, _config) = match init() {
        Ok((conn, config)) => (conn, config),
        Err(e) => {
            return Err(format!("Error initializing: {}", e));
        }
    };

    let mut ip_address = "0.0.0.0".to_string(); // Set a default value for `ip_address`

    if let Some(ip) = public_ip::addr().await {
        println!("public ip address: {:?}", ip);
        ip_address = ip.to_string(); // Assign the IP address to the variable
    }


    let current_user: User = loop {
        match conn.query_row("SELECT * FROM users", [], |row| {
            Ok(User {
                login: row.get(0)?,
                name: row.get(1)?,
                public_key: row.get(2)?,
                ip_address: row.get(3)?,
                port: row.get(4)?,
            })
        }) {
            Ok(user) => break user,
            Err(_) => println!("No user found"),
        }
    };

    update_user(&conn, &current_user, ip_address).expect("TODO: panic message");

    Ok(current_user)
}

#[tauri::command]
async fn init_register_user(permanent_login: String, display_name: String, port_str: String) -> Result<(), String> {
    let (conn, _config) = match init() {
        Ok((conn, config)) => (conn, config),
        Err(e) => {
            println!("Error initializing: {}", e);
            return Err("Error initializing database".to_string());
        }
    };

    let mut ip_address = "0.0.0.0".to_string(); // Set a default value for `ip_address`

    if let Some(ip) = public_ip::addr().await {
        println!("public ip address: {:?}", ip);
        ip_address = ip.to_string(); // Assign the IP address to the variable
    }

    let mut port: u16 = 1096;

    if !port_str.is_empty() {
        loop {
            match port_str.parse::<u16>() {
                Ok(p) => {
                    port = p;
                    break;
                },
                Err(_) => {
                    println!("Invalid port number, please enter a valid port:");
                }
            }
        }
    }

    let (public_key, _private_key) = generate_keypair();

    let insert_sql = "INSERT INTO users (login, name, public_key, ip_address, port) VALUES (?, ?, ?, ?, ?)";
    conn.execute(insert_sql, params![permanent_login, display_name, public_key, ip_address, port]).map_err(|e| format!("Failed to execute query: {}", e))?;

    Ok(())
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_user_data])
        .invoke_handler(tauri::generate_handler![init_register_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
