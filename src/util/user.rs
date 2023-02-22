use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

/* idk what this code does, but it's needed for the User struct to work */
#[derive(Serialize, Deserialize, Debug)]

/* this is the User struct */
pub struct User {
    pub(crate) login: String,
    pub(crate) name: String,
    pub(crate) public_key: String,
    pub(crate) ip_address: Ipv4Addr,
    pub(crate) port: u16,
}

/* this is the constructor for the User struct */
impl User {
    pub fn new(login: String, name: String, public_key: String, ip_address: Ipv4Addr, port: Option<u16>) -> Self {
        User {
            login,
            name,
            public_key,
            ip_address,
            port: port.unwrap_or(1096),
        }
    }
}
