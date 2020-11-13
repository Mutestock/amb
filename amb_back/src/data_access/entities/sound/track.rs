use chrono::naive::{NaiveDateTime};

//use crate::schema::tracks;
//use crate::schema::tracks::dsl;
//use crate::schema::tracks::dsl::*;

use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::PgConnection;
use serde_derive::{Deserialize, Serialize};



// Read them.
// 1. diesel.rs/guides/getting-started/
// 2. https://github.com/steadylearner/Rust-Full-Stack/blob/master/actix/src/database/models/product.rs

// $echo DATABASE_URL=trackgres://trackgres:trackgres@localhost/warp > .env

// $diesel setup
// $diesel migration create_track
// Move to migrations/ folder.
// CREATE TABLE tracks (
//   id SERIAL PRIMARY KEY,
//   title VARCHAR NOT NULL,
//   body TEXT NOT NULL
// )
// $diesel migration run



#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="track"]
pub struct NewTrack {
    pub title: String,
    pub body: String,
}

impl NewTrack {
    pub fn create(&self, connection: &PgConnection) -> Result<track, diesel::result::Error> {
        diesel::insert_into(tracks::table)
            .values(self)
            .get_result(connection)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Track{
    pub id: i32,
    pub title: String,
    pub duration: f32,
    pub description: String,
    pub credits: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Track {
    pub fn find(track_id: &i32, connection: &PgConnection) -> Result<track, diesel::result::Error> {
        tracks::table.find(track_id).first(connection)
    }

    pub fn delete(track_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(dsl::tracks.find(track_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(track_id: &i32, new_track: &Newtrack, connection: &PgConnection) -> Result<(), diesel::result::Error> {

        diesel::update(dsl::tracks.find(track_id))
            .set(new_track)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct trackList(pub Vec<track>);

impl trackList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = tracks
            .limit(10)
            .load::<track>(connection)
            .expect("Error loading tracks");
        trackList(result)
    }
}