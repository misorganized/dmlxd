mod init;

use crate::init::init;
use crate::util::user::{update_user, User};

mod util {
    pub mod timer;
    pub mod user;
    pub mod register;
}

mod comms {
    pub mod init_comm;
}

#[tauri::command]
async fn get_user_data() -> Result<User, String> {
    let (conn, _config) = match init() {
        Ok((conn, config)) => (conn, config),
        Err(e) => {
            return Err(format!("Error initializing: {}", e));
        }
    };

    let mut ip_address = "0.0.0.0".to_string(); // Set a default value for `ip_address`

    if let Some(ip) = public_ip::addr().await {
        println!("public ip address: {:?}", ip);
        ip_address = ip.to_string(); // Assign the IP address to the variable
    }


    let current_user: User = loop {
        match conn.query_row("SELECT * FROM users", [], |row| {
            Ok(User {
                login: row.get(0)?,
                name: row.get(1)?,
                public_key: row.get(2)?,
                ip_address: row.get(3)?,
                port: row.get(4)?,
            })
        }) {
            Ok(user) => break user,
            Err(_) => println!("No user found"),
        }
    };

    update_user(&conn, &current_user, ip_address).expect("TODO: panic message");

    Ok(current_user)
}


fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
