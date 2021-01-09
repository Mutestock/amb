use crate::schema::tracks;
use crate::schema::tracks::dsl;
use crate::schema::tracks::dsl::*;

use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use serde_derive::{Deserialize, Serialize};
use std::{fs, time::SystemTime};

//use uuid::Uuid;

#[derive(Insertable, Deserialize, AsChangeset, PartialEq, Debug)]
#[table_name = "tracks"]
pub struct NewTrack {
    pub user_id: i32,
    pub title: String,
    pub path: String,
    pub uuid_fname: String,
    pub description: Option<String>,
    pub duration: i32,
    pub credits: String,
}

#[derive(Deserialize)]
pub struct NewTrackReception {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub duration: i32,
    pub credits: String,
}

impl NewTrackReception {
    pub fn to_new_track(&self, _uuid: String, p: String) -> NewTrack {
        NewTrack {
            user_id: self.user_id,
            title: self.title.to_string(),
            uuid_fname: _uuid,
            path: p,
            description: self.description.to_owned(),
            duration: self.duration,
            credits: self.credits.to_string(),
        }
    }
}

fn create_path(new_track: &NewTrack) {
    fs::create_dir_all("/usr/resources/{}/{}").expect("Could not create directory");
}

impl NewTrack {
    pub fn create(&self, connection: &PgConnection) -> Result<Track, diesel::result::Error> {
        println!("{:#x?}", &self);
        diesel::insert_into(tracks::table)
            .values(self)
            .get_result(connection)
    }
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Track {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub uuid_fname: String,
    pub path: String,
    pub description: Option<String>,
    pub uploaded_at: Option<SystemTime>,
    pub duration: Option<i32>,
    pub credits: String,
}

impl Track {
    pub fn find(track_id: &i32, connection: &PgConnection) -> Result<Track, diesel::result::Error> {
        tracks::table.find(track_id).first(connection)
    }

    pub fn delete(track_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(dsl::tracks.find(track_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(
        track_id: &i32,
        new_track: &NewTrack,
        connection: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(dsl::tracks.find(track_id))
            .set(new_track)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackList(pub Vec<Track>);

impl TrackList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = tracks
            .limit(10)
            .load::<Track>(connection)
            .expect("Error loading tracks");
        TrackList(result)
    }
}
