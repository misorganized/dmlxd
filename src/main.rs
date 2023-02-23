mod init;

use rusqlite::Connection;
use crate::init::init;
use crate::util::register::{list_contacts, read_input, register_contact, register_user};
use crate::util::user::{update_user, User};
use crate::comms::init_comm;

mod util {
    pub mod timer;
    pub mod user;
    pub mod register;
}

mod comms {
    pub mod init_comm;
}

#[macro_use]
extern crate log;
#[tokio::main]
async fn main() {
    let (conn, config) = match init() {
        Ok((conn, config)) => (conn, config),
        Err(e) => {
            eprintln!("Error initializing: {}", e);
            return;
        }
    };

    println!("Config: {:?}", config);

    if config.first_run {
        get_user_data(&conn).await;
    }

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
            Err(_) => get_user_data(&conn).await,
        }
    };

    update_user(&conn, &current_user, ip_address).expect("TODO: panic message");

    init_comm::init_connection(&current_user).expect("TODO: panic message");


    println!("\nWelcome to the chat client! Type 'exit' to exit.\n");

    loop {
        let command = read_input();
        let result_vec: Vec<&str> = command.split_whitespace().collect();

        let command = result_vec.get(0).unwrap_or(&"");
        let arguments = match result_vec.get(1..) {
            Some(args) => args,
            None => &[],
        };

        match command {
            &"exit" => break,
            &"list_contacts" => list_contacts(&conn).expect("TODO: panic message"),
            &"add_contact" =>
                if arguments.len() != 2 {
                    println!("\nThe correct usage is: add_contact <login> <ip_address>");
                } else if arguments[0] == "help" {
                    println!("\nThe correct usage is: add_contact <login> <ip_address>");
                } else {
                    register_contact(&conn,
                                     arguments[0],
                                     arguments[1]).expect("TODO: panic message");
                    println!("Contact added successfully!");
                },

            _ => println!("Unknown command: {}", command),
        }
    }

    drop(conn);
}

async fn get_user_data(conn: &Connection) {
    // Prompt the user for input
    println!("First run detected. Registering user...\n\n");

    println!("Please enter your permanent login:");
    let permanent_login = read_input();

    println!("Please enter your display name:");
    let display_name = read_input();

    register_user(&conn, permanent_login, display_name).await.expect("TODO: panic message");
}

