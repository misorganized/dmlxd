mod init;

use crate::init::init;
use crate::util::register::{list_contacts, read_input, register_contact, register_user};

mod util {
    pub mod timer;
    pub mod user;
    pub mod register;
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
        register_user(&conn).await.expect("TODO: panic message");
    }

    println!("\nWelcome to the chat client! Type 'exit' to exit.\n");

    loop {
        let command = read_input();
        let command_str = match command {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        };
        let result_vec: Vec<&str> = command_str.split_whitespace().collect();

        let command = result_vec.get(0).unwrap_or(&"");
        let arguments = match result_vec.get(1..) {
            Some(args) => args,
            None => &[],
        };

        match command {
            &"exit" => break,
            &"list_contacts" => list_contacts(&conn).expect("TODO: panic message"),
            &"add_contact" =>
                if arguments.len() == 0 {
                    println!("\nThe correct usage is: add_contact <login> <ip_address>");
                } else if arguments[0] == "help" {
                    println!("\nThe correct usage is: add_contact <login> <ip_address>");
                } else if arguments.len() != 2 {
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
