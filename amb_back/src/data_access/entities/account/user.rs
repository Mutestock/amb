use chrono::naive::{NaiveDateTime};

pub struct User{
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_login: NaiveDateTime,
    pub admin: bool,
}

pub struct NewUser{
    pub username: String,
    pub password: String,
    pub email: String,
    pub description: String,
    pub admin: bool,
}

