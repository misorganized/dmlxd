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
