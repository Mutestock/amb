use chrono::naive::{NaiveDateTime};

pub struct Image{
    pub id: i32,
    pub title: String,
    pub upload_date: NaiveDateTime,
    pub description: String,
}