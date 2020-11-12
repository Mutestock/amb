use chrono::naive::{NaiveDateTime};

pub struct TrackTag{
    pub id: i32,
    pub name: String;
    pub creation_date: NaiveDateTime,
}
