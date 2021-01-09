use chrono::naive::{NaiveDateTime};

use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::PgConnection;
use serde_derive::{Deserialize, Serialize};

pub struct NewPlaylist{
    pub name: String,
    pub description: String,
}


impl NewPlaylist{
    pub fn create(&self, connection: &PgConnection) -> Result<Playlist, diesel::result::Error>{
        diesel::insert_into(playlists::table)
            .values(self)
            .get_results(connection)
    }
}

pub struct Playlist{
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub description: String,
}


impl Playlist{
    pub fn find(playlist_id: &i32, connection: &PgConnection) -> Result<playlist, diesel::result::Error>{
        playlists::table.find(playlist_id).first(connection)
    }

    pub fn delete(playlist_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::delete(dsl::playlists.find(playlist_id)).execute(connection)?;
    }

    pub fn update(playlist_id: &i32, new_playlist: &NewPlaylist, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::update(dsl::playlists::find(playlist_id))
            .set(new_playlist)
            .execute(connection)?;
        Ok(())
    }
}

#![derive(Serialize, Deserialize, Debug)]
pub struct PlaylistList(pub Vec<Playlist>)

impl PlaylistList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = playlists
            .limit(10)
            .load::<playlist>(connection)
            .expect("Error loading playlists");
        PlaylistList(result)
    }
}
