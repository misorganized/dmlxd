mod init;
use crate::init::init;

mod util {
    pub mod timer;
}

fn main() {
    let (conn, config) = init().expect("Error loading...");
    println!("Config: {:?}", config);
}
