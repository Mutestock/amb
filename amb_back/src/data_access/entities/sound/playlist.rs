use chrono::naive::{NaiveDateTime};

pub struct Playlist{
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub description: String,
}