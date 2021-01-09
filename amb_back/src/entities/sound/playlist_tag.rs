use chrono::naive::{NaiveDateTime};

pub struct PlaylistTag {
    pub id: i32,
    pub name: String,
    pub creation_date: Option<SystemTime>,
}

pub struct NewPlaylistTag {
    pub name: String
}

