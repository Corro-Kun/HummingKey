use rusqlite::params;
use crate::db::{connect, create_db};
use crate::models::*;
use bcrypt::{hash, verify, DEFAULT_COST};

#[tauri::command]
pub fn verify_db() -> bool{
    let conn = connect();
    let mut value = String::new();

    let result = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
        .and_then(|mut stmt|{
            stmt.query_row(&["user"], |row| Ok(value = row.get(0)?))
        })
        .is_ok();

    if !result{
        create_db();
        return false;
    }

    true
}

#[tauri::command]
pub fn create_user(user: User) -> bool{
    let conn = connect();

    let hashed_password = hash(user.password, DEFAULT_COST).unwrap();

    let _ = conn.execute("INSERT INTO user (id, name, password) VALUES (?1, ?2, ?3)",
        params![1, user.name, hashed_password]
    );

    true
}

#[tauri::command]
pub fn get_name_user() -> String{
    let conn = connect();
    let mut name = String::new();

    let mut stmt = conn.prepare("SELECT name FROM user").map_err(|err| format!("the error is {}", err.to_string())).unwrap();

    let _ = stmt.query_row([], |row| {
        name = row.get(0)?;
        Ok(())
    });

    name
}

#[tauri::command]
pub fn login(password: String) -> bool{
    let conn = connect();
    let mut hashed_password = String::new();

    let mut stmt = conn.prepare("SELECT password FROM user").map_err(|err| format!("the error is {}", err.to_string())).unwrap();

    let _ = stmt.query_row([], |row| {
        hashed_password = row.get(0)?;
        Ok(())
    });

    verify(password, &hashed_password).unwrap()
}