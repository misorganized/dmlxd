mod init;
use crate::init::init;

mod util {
    pub mod timer;
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

    drop(conn);
}
