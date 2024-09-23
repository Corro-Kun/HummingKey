use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User{
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Password{
    pub id: i32,
    pub name: String,
    pub icon: i32,
    pub user: String,
    pub user_length: i32,
    pub password: String,
    pub password_length: i32,
}