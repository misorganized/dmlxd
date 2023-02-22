use std::net::Ipv4Addr;
use crate::util::user::User;

mod util {
    pub mod user;
}

fn main() {
    /* random user code */
    let user = User::new("john123".to_string(),
                         "John".to_string(),
                         "public_key".to_string(),
                         Ipv4Addr::new(127, 0, 0, 1),
                         Some(8080)
    );

    println!("ID: {}", user.login);
    println!("Name: {}", user.name);
    println!("Public Key: {}", user.public_key);
    println!("IP Address: {}", user.ip_address);
    println!("Port: {}", user.port);
}
