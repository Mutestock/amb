
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::PgConnection;
use serde_derive::{Deserialize, Serialize};

use crate::data_access::entities::account::user::User;
use crate::schema::images;
use crate::schema::images::dsl;
use crate::schema::images::dsl::*;


use std::time::SystemTime;

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Image{
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub path: String,
    pub description: Option<String>,
    pub upload_date: Option<SystemTime>,
}

#[derive(Insertable, Deserialize, AsChangeset, Associations)]
#[belongs_to(User)]
#[table_name="images"]
pub struct NewImage{
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub path: Option<String>,
}


impl NewImage {
    pub fn create(&self, connection: &PgConnection) -> Result<Image, diesel::result::Error> {
        // The path is not nullable. At the same time, you shouldn't be able to manually insert the path.
        // The path must be constructed based on other factors, such as the connected user's information.
        // img + usr_id = ~/resources/images/username/upload_date/image_name
        let with_path = NewImage{
            user_id: self.user_id,
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
    pub fn find(image_id: &i32, connection: &PgConnection) -> Result<Image, diesel::result::Error>{
        images::table.find(image_id).first(connection)
    }

    pub fn delete(image_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::delete(dsl::images.find(image_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(image_id: &i32, new_image: &NewImage, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::update(dsl::images.find(image_id))
            .set(new_image)
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
