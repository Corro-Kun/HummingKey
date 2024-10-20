// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod command;
mod db;
mod lib;

use command::*;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      verify_db, create_user, get_name_user, get_image_user,login, create_password, get_passwords, 
      descrypt_data, get_password_by_id, delete_password, update_password, update_user_without_password,
      update_user_with_password,
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
