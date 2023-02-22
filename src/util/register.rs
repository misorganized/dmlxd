use std::error::Error;
use std::io;

use sodiumoxide::crypto::box_::gen_keypair;
use rusqlite::{params, Connection};
use sodiumoxide::hex;

pub async fn register_user(conn: &Connection) -> Result<(), Box<dyn Error>> {
    // Prompt the user for input
    println!("Please enter your permanent login:");
    let permanent_login = read_input()?;

    println!("Please enter your display name:");
    let display_name = read_input()?;

    //println!("Please enter your IP address:");
    let mut ip_address = "0.0.0.0".to_string(); // Set a default value for `ip_address`

    if let Some(ip) = public_ip::addr().await {
        println!("public ip address: {:?}", ip);
        ip_address = ip.to_string(); // Assign the IP address to the variable
    } else {
        println!("Unable to get public IP address");
    }

    println!("Please enter your port number (default is 1096):");
    let mut port_str = read_input()?;
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
                    port_str = read_input()?;
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

fn generate_keypair() -> (String, String) {
    // Generate a new keypair using sodiumoxide
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

fn read_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // Trim the input to remove the newline character
    Ok(input.trim().to_string())
}
