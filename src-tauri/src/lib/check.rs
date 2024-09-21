use rusqlite::Connection;
use crate::db::create_db;

pub fn check_db(conn: &mut Connection){
    let mut value = String::new();

    let result = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
        .and_then(|mut stmt|{
            stmt.query_row(&["user"], |row| Ok(value = row.get(0)?))
        })
        .is_ok();

    if !result{
        create_db();
    }
}