use rusqlite::params;
use crate::db::connect;
use crate::models::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::lib::check::check_db;
use crate::lib::encrypt::{encrypt, decrypt, new_key};

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

    let _ = conn.execute("INSERT INTO user (id, name, password, image) VALUES (?1, ?2, ?3, ?4)",
        params![1, user.name, hashed_password, user.image]
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
pub fn get_image_user() -> String{
    let conn = connect();
    let mut img = String::new();

    let mut stmt = conn.prepare("SELECT image FROM user").map_err(|err| format!("the error is {}", err.to_string())).unwrap();

    let _ = stmt.query_row([], |row|{
        img = row.get(0)?;
        Ok(())
    });

    img
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

    let cipher = new_key(&password);

    let user = encrypt(&new_password.user, &cipher);
    let password = encrypt(&new_password.password, &cipher);

    let _ = conn.execute("INSERT INTO password (name, icon, user, user_length, password, password_length) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", params![new_password.name, new_password.icon, user, new_password.user.len(),password, new_password.password_length]).expect("error while inserting password");

    true
}

#[tauri::command]
pub fn get_passwords() -> Vec<Password>{
    let conn = connect();
    let mut stmt = conn.prepare("SELECT id, name, icon, user, user_length, password, password_length FROM password").map_err(|err| format!("the error is {}", err.to_string())).expect("error while preparing statement");
    let mut passwords = Vec::new();

    stmt.query_map([], |row| {
        Ok(Password{
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
            user: row.get(3)?,
            user_length: row.get(4)?,
            password: row.get(5)?,
            password_length: row.get(6)?
        })
    }).unwrap().for_each(|password| {
        passwords.push(password.unwrap());
    });

    passwords
}

#[tauri::command]
pub fn descrypt_data(password: String, data: String) -> String{
    let cipher = new_key(&password);

    decrypt(&data, &cipher)
}

#[tauri::command]
pub fn get_password_by_id(id: i32, password: String) -> Password{
    let conn = connect();

    let cipher = new_key(&password);

    let mut stmt = conn.prepare("SELECT id, name, icon, user, user_length, password, password_length FROM password WHERE id = ?1").map_err(|err| format!("the error is {}", err.to_string())).expect("error while preparing statement");

    let password = stmt.query_row(params![id], |row| {
        Ok(Password{
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
            user: decrypt(&row.get(3)?, &cipher),
            user_length: row.get(4)?,
            password: decrypt(&row.get(5)?, &cipher),
            password_length: row.get(6)?
        })
    }).expect("error while getting password");

    password
}

#[tauri::command]
pub fn update_password(update_password: Password, password: String){
    let conn = connect();

    let cipher = new_key(&password);

    let user = encrypt(&update_password.user, &cipher);
    let password = encrypt(&update_password.password, &cipher);

    let _ = conn.execute("UPDATE password SET name = ?1, icon = ?2, user = ?3, user_length = ?4, password = ?5, password_length = ?6 WHERE id = ?7", params![update_password.name, update_password.icon, user, update_password.user_length, password, update_password.password_length, update_password.id]).expect("error while updating password");
}

#[tauri::command]
pub fn delete_password(id: i32){
    let conn = connect();

    let _ = conn.execute("DELETE FROM password WHERE id = ?1", params![id]).expect("error while deleting password");
}
