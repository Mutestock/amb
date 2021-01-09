use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use serde_derive::{Deserialize, Serialize};

// These variables are to be determined in accordance with the functionalities which tone.js supports in the front end.
// And, in extension, which of said functionalities this project should support.
// Refer to: https://tonejs.github.io/docs/
// All the more advanced options will have a default value which makes them practically disabled.


#[derive(Insertable, Deserialize, AsChangeset)]
pub struct NewTrackConfiguration {
    pub playlist_id: i32,
    pub track_id: i32,
    pub is_looping: bool,
    pub is_volume_random: bool,
    pub is_interval_random: bool,
    pub volume_min: f32,
    pub volume_max: f32,
    pub interval_min: i32,
    pub interval_max: i32,
    pub pan_x: i32,
    pub pan_y: i32,
    pub pan_z: i32,
    pub pitch_shift: i32,
    pub reverb: i32,
    pub compressor_threshold: i32,
    pub compressor_gain_reduction: i32,
}

#[derive(Deserialize)]
pub struct TrackConfiguration {
    pub id: i32,
    pub playlist_id: i32,
    pub track_id: i32,
    pub is_looping: bool,
    pub is_volume_random: bool,
    pub is_interval_random: bool,
    pub volume_min: f32,
    pub volume_max: f32,
    pub interval_min: i32,
    pub interval_max: i32,
    pub pan_x: i32,
    pub pan_y: i32,
    pub pan_z: i32,
    pub pitch_shift: i32,
    pub reverb: i32,
    pub compressor_threshold: i32,
    pub compressor_gain_reduction: i32,
}

impl TrackConfiguration{
    pub fn find(track_configuration_id: &i32, connection: &Pgconnection) -> Result<TrackConfiguration, diesel::result::Error>{
        track_configurations::table.find(track_configuration_id).first(connection)
    }

    pub fn delete(track_configuration_id: &i32, connection: &Pgconnection) -> Result<TrackConfiguration, diesel::result::Error>{
        diesel::delete(dsl::track_configurations.find(track_configuration_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(track_configuration_id: &i32, new_track_configuration: &NewTrackConfiguration, connection: &Pgconnection) -> Result<TrackConfiguration, diesel::result::Error>{
        diesel::update(dsl::track_configurations.find(track_configuration_id))
            .set(new_track_configuration)
            .execute(connection)?;
        Ok(())
    }
}

impl NewTrackConfiguration{
    pub fn create(&self, connection: &Pgconnection) -> Result<TrackConfiguration, diesel::result::Error>{
        diesel::insert_into(track_configurations::table)
            .values(self)
            .get_result(connection)
    }
}

// Need to specify it to only show configurations of the same track.
// Used to gain inspiration of other peoples' configurations.
pub struct TrackConfigurationList(pub Vec<TrackConfiguration>);

impl TrackConfigurationList{
    pub fn list(connection: &Pgconnection) -> Self {
        let result = TrackConfigurations
            .limit(10)
            .load::<TrackConfiguration>(connection)
            .expect("Error loading track configurations");
        TrackConfigurationList(result)
    }
}