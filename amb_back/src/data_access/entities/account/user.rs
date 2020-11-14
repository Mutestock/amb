use chrono::naive::{NaiveDateTime};

use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::PgConnection;
use serde_derive::{Deserialize, Serialize};


pub struct NewUser{
    pub username: String,
    pub password: String,
    pub email: String,
    pub description: String,
    pub admin: bool,
}

impl NewUser{
    pub fn create(&self, connection: &PgConnection) -> Result<User, diesel::result::Error>{
        diesel::insert_into(users::table)
            .values(self)
            .get_results(connection)
    }
}


pub struct User{
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_login: NaiveDateTime,
    pub admin: bool,
}


impl User{
    pub fn find(user_id: &i32, connection: &PgConnection) -> Result<user, diesel::result::Error>{
        users::table.find(user_id).first(connection)
    }

    pub fn delete(user_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::delete(dsl::users.find(user_id)).execute(connection)?;
    }

    pub fn update(user_id: &i32, new_user: &NewUser, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::update(dsl::users::find(user_id))
            .set(new_user)
            .execute(connection)?;
        Ok(())
    }
}

#![derive(Serialize, Deserialize, Debug)]
pub struct UserList(pub Vec<User>)

impl UserList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = users
            .limit(10)
            .load::<user>(connection)
            .expect("Error loading users");
        UserList(result)
    }
}
