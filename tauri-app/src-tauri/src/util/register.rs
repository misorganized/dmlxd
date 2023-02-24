use std::error::Error;
use std::io;

use sodiumoxide::crypto::box_::gen_keypair;
use rusqlite::{params, Connection};
use sodiumoxide::hex;
use crate::init::db_conn;
use crate::util::user::User;

pub async fn register_user(permanent_login: String, display_name: String) -> Result<(), Box<dyn Error>> {

    let conn: Connection = db_conn();

    let mut ip_address = "0.0.0.0".to_string(); // Set a default value for `ip_address`

    if let Some(ip) = public_ip::addr().await {
        println!("public ip address: {:?}", ip);
        ip_address = ip.to_string(); // Assign the IP address to the variable
    }

    println!("Please enter your port number (default is 1096):");
    let mut port_str = read_input();
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
                    port_str = read_input();
                }
            }
        }
    }

    // Generate a new public/private key pair for the user
    let (public_key, _private_key) = generate_keypair();

    // Insert the user's information into the database
    let insert_sql = "INSERT INTO users (login, name, public_key, ip_address, port) VALUES (?, ?, ?, ?, ?)";
    conn.execute(insert_sql, params![permanent_login, display_name, public_key, ip_address, port])?;
    Ok(())
}

pub fn register_contact(conn: &Connection, login: &str, ip_address: &str) -> Result<(), Box<dyn Error>> {
    // Insert the contact's information into the database

    let name = "Unknown"; // Set a default value for `name`
    let public_key = "Unknown"; // Set a default value for `public_key`
    let port = 1096; // Set a default value for `port`

    /* TODO: Contact the user with the give credentials and get their information */

    let insert_sql = "INSERT INTO contacts (login, name, public_key, ip_address, port)\
     VALUES (?, ?, ?, ?, ?)";
    conn.execute(insert_sql, params![login, name, public_key, ip_address, port])?;

    Ok(())
}

pub fn list_contacts (conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT login, name, public_key, ip_address, port FROM contacts")?;
    let mut contacts_iter = stmt.query_map(params![], |row| {
        Ok(User {
            login: row.get(0)?,
            name: row.get(1)?,
            public_key: row.get(2)?,
            ip_address: row.get(3)?,
            port: row.get(4)?,
        })
    })?;

    if let Some(first_contact) = contacts_iter.next() {
        println!("Found contact: {:?}", first_contact);

        for contact in contacts_iter {
            println!("Found contact: {:?}", contact);
        }
    } else {
        println!("No contacts found");
    }


    Ok(())
}

pub(crate) fn generate_keypair() -> (String, String) {
    // Generate a new keypair using sodium oxide
    let (public_key, private_key) = gen_keypair();

    // Convert the keys to hexadecimal strings for storage
    let public_key_str = hex::encode(public_key.as_ref());
    let private_key_str = hex::encode(private_key.as_ref());

    (public_key_str, private_key_str)
}

/*
fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    // Hash the password using bcrypt
    let hashed_password = hash(password, 10)?;
    Ok(hashed_password)
}
 */

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("TODO: panic message");
    // Trim the input to remove the newline character
    return input.trim().to_string();
}


/* Unit tests */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_keypair() {
        let (public_key, private_key) = generate_keypair();
        assert_eq!(public_key.len(), 64);
        assert_eq!(private_key.len(), 64);
    }
}