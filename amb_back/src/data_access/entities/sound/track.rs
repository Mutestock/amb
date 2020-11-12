use chrono::naive::{NaiveDateTime};

pub struct Track{
    pub id: i32,
    pub title: String,
    pub duration: f32,
    pub description: String,
    pub credits: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}