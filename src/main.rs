mod init;
use crate::init::init;
use crate::init::first_login;

mod util {
    pub mod timer;
    pub mod user;
}

fn main() {
    let (conn, config) = match init() {
        Ok((conn, config)) => (conn, config),
        Err(e) => {
            eprintln!("Error initializing: {}", e);
            return;
        }
    };

    println!("Config: {:?}", config);

    if config.first_run {
        first_login(&conn);
    }

    drop(conn);
}
