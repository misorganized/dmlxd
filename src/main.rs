mod init;
use crate::init::init;
use crate::util::register::register_user;

mod util {
    pub mod timer;
    pub mod user;
    pub mod register;
}

#[tokio::main]
async fn main() {
    let (conn, config) = match init() {
        Ok((conn, config)) => (conn, config),
        Err(e) => {
            eprintln!("Error initializing: {}", e);
            return;
        }
    };

    println!("Config: {:?}", config);

    if config.first_run {
        register_user(&conn).await.expect("TODO: panic message");
    }

    drop(conn);
}
