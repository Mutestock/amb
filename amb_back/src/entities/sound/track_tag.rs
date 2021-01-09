use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::PgConnection;
use serde_derive::{Deserialize, Serialize};

use std::time::SystemTime;

pub struct NewTrackTag{
    pub name: String,
    pub description: String,
}


impl NewTrackTag{
    pub fn create(&self, connection: &PgConnection) -> Result<TrackTag, diesel::result::Error>{
        diesel::insert_into(track_tags::table)
            .values(self)
            .get_results(connection)
    }
}

pub struct TrackTag{
    pub id: i32,
    pub name: String,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub description: Option<String>,
}


impl TrackTag{
    pub fn find(track_tag_id: &i32, connection: &PgConnection) -> Result<track_tag, diesel::result::Error>{
        track_tags::table.find(track_tag_id).first(connection)
    }

    pub fn delete(track_tag_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::delete(dsl::track_tags.find(track_tag_id)).execute(connection)?;
    }

    pub fn update(track_tag_id: &i32, new_track_tag: &NewTrackTag, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::update(dsl::track_tags::find(track_tag_id))
            .set(new_track_tag)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackTagList(pub Vec<TrackTag>)

impl TrackTagList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = track_tags
            .limit(10)
            .load::<track_tag>(connection)
            .expect("Error loading track_tags");
        TrackTagList(result)
    }
}
