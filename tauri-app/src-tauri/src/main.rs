#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use std::error::Error;
use std::path::PathBuf;
use crate::database;
use crate::util::user;




// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn register_user(permanent_login: String, display_name: String, ip_address: String, port:i64) -> Result<(), Box<dyn Error>> {
    let conn = database::load_db_helper()?;
    database::register_user_helper(&conn, permanent_login, display_name, ip_address, port)?;
    Ok((()))
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("Hello, world!");
}
