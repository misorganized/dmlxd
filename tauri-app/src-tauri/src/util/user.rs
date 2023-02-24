use std::error::Error;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

/* idk what this code does, but it's needed for the User struct to work */
#[derive(Serialize, Deserialize, Debug)]

/* this is the User struct */
pub struct User {
    pub(crate) login: String,
    pub(crate) name: String,
    pub(crate) public_key: String,
    pub(crate) ip_address: String,
    pub(crate) port: i32,
}

pub fn update_user(conn: &Connection, user: &User, ip: String) -> Result<(), Box<dyn Error>> {
    let update_sql = "UPDATE users SET name = ?, public_key = ?, ip_address = ?, port = ? WHERE login = ?";
    conn.execute(update_sql, params![user.name, user.public_key, ip, user.port, user.login])?;
    Ok(())
}
