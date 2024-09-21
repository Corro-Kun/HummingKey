use rusqlite::Connection;

pub fn connect() -> Connection {
    let conn = Connection::open("passwords.db").unwrap();
    conn 
}

pub fn create_db() {
    let conn = Connection::open("passwords.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    ).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS password (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            icon INTEGER NOT NULL,
            user TEXT NOT NULL,
            password TEXT NOT NULL,
            password_length INTEGER NOT NULL
        )",
        [],
    ).unwrap();
}