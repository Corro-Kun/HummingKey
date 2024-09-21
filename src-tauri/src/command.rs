use rusqlite::params;
use crate::db::connect;
use crate::models::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::lib::check::check_db;
use aes::cipher::{generic_array::GenericArray, KeyInit};
use aes::cipher::{BlockDecrypt, BlockEncrypt};
use aes::Aes256;
use block_padding::generic_array::functional::FunctionalSequence;

#[tauri::command]
pub fn verify_db() -> bool{
    let mut conn = connect();
    
    check_db(&mut conn);

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM user").map_err(|err| format!("the error is {}", err.to_string())).unwrap();

    let count: usize = stmt.query_row([], |row| row.get(0)).unwrap();

    return count > 0
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

#[tauri::command]
pub fn create_password(new_password: Password, password: String) -> bool{
    let conn = connect();

    let mut key = GenericArray::from([0u8; 32]);
    let password_bytes = password.as_bytes();
    let mut index = 0;

    key = key.map(|mut x|{
        if password_bytes.len() > index{
            x = password_bytes[index];
            index += 1;
        }
        x
    });

    let password_new_bytes = new_password.password.as_bytes();

    let mut block = GenericArray::from([0u8; 16]);

    index = 0;

    block = block.map(|mut x|{
        if password_new_bytes.len() > index{
            x = password_new_bytes[index];
            index += 1;
        }
        x
    });

    let cipher = Aes256::new(&key);

    cipher.encrypt_block(&mut block);

    let _ = conn.execute("INSERT INTO password (name, icon, user, password, password_length) VALUES (?1, ?2, ?3, ?4, ?5)", params![new_password.name, new_password.icon, new_password.user, hex::encode(block), new_password.password_length]).expect("error while inserting password");

    true
}

#[tauri::command]
pub fn get_passwords() -> Vec<Password>{
    let conn = connect();
    let mut stmt = conn.prepare("SELECT id, name, icon, user, password, password_length FROM password").map_err(|err| format!("the error is {}", err.to_string())).unwrap();
    let mut passwords = Vec::new();

    stmt.query_map([], |row| {
        Ok(Password{
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
            user: row.get(3)?,
            password: row.get(4)?,
            password_length: row.get(5)?
        })
    }).unwrap().for_each(|password| {
        passwords.push(password.unwrap());
    });

    passwords
}