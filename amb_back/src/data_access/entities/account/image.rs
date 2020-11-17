use chrono::naive::{NaiveDateTime};

use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::PgConnection;
use serde_derive::{Deserialize, Serialize};

use crate::schema::images;
use crate::schema::images::dsl;
use crate::schema::images::dsl::*;

use std::time::SystemTime;

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Image{
    pub id: i32,
    pub title: String,
    pub path: String,
    pub description: Option<String>,
    pub upload_date: Option<SystemTime>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="images"]
pub struct NewImage{
    pub title: String,
    pub description: String,
    pub path: Option<String>,
}

fn assign_path(new_image: &NewImage) -> NewImage{

    NewImage{
        title: new_image.title.to_string(),
        description: new_image.description.to_string(),
        path:Some(String::from("cake")),
    }
}


impl NewImage {
    pub fn create(&self, connection: &PgConnection) -> Result<Image, diesel::result::Error> {
        // The path is not nullable. At the same time, you shouldn't be able to manually insert the path.
        // The path must be constructed based on other factors, such as the connected user's information.
        // img + usr_id = ~/resources/images/username/upload_date/image_name
        //let cloned = self.clone();
        let with_path = NewImage{
            title: self.title.to_string(),
            description: self.description.to_string(),
            path: Some(String::from("cake"))
        };
        //let with_path = assign_path(self);

        diesel::insert_into(images::table)
            .values(with_path)
            .get_result(connection)
    }
}


impl Image{
    pub fn find(user_id: &i32, connection: &PgConnection) -> Result<Image, diesel::result::Error>{
        images::table.find(user_id).first(connection)
    }

    pub fn delete(user_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::delete(dsl::images.find(user_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(user_id: &i32, new_user: &NewImage, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::update(dsl::images.find(user_id))
            .set(new_user)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageList(pub Vec<Image>);

impl ImageList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = images
            .limit(10)
            .load::<Image>(connection)
            .expect("Error loading posts");
            ImageList(result)
    }
}
