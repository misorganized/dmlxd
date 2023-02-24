use std::error::Error;
use crate::util::user::User;
use reqwest::get;
use serde::Deserialize;

pub fn init_connection(user: &User) -> Result<(), Box<dyn Error>> {
    // Generate a new keypair for this node
    let internal_user = user.clone();
    let keypair = &internal_user.public_key;
    println!("Public key: {:?}", keypair);

    Ok(())
}