use chrono::naive::NaiveDateTime;

use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::PgConnection;
use serde_derive::{Deserialize, Serialize};
use diesel::pg::Pg;

use crate::schema::users;
use crate::schema::users::dsl;
use crate::schema::users::dsl::*;

use std::time::SystemTime;


#[derive(Insertable, Deserialize, AsChangeset, PartialEq)]
#[table_name="users"]
pub struct NewUser{
    pub username: String,
    pub password: String,
    pub email: String,
    pub description: Option<String>,
    pub admin: bool,
}

impl NewUser {
    pub fn create(&self, connection: &PgConnection) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(self)
            .get_result(connection)
    }
}


//impl NewUser{
//    pub fn create(&self, connection: &PgConnection) -> Result<User, diesel::result::Error>{
//        diesel::insert_into(users::table)
//            .values(self)
//            .get_results(connection)
//    }
//}

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub description: Option<String>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub last_login: Option<SystemTime>,
    pub admin: bool,
}


impl User{
    pub fn find(user_id: &i32, connection: &PgConnection) -> Result<User, diesel::result::Error>{
        users::table.find(user_id).first(connection)
    }

    pub fn delete(user_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::delete(dsl::users.find(user_id)).execute(connection)?;
        Ok(())
    }

    pub fn update(user_id: &i32, new_user: &NewUser, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::update(dsl::users.find(user_id))
            .set(new_user)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserList(pub Vec<User>);

impl UserList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = users
            .limit(10)
            .load::<User>(connection)
            .expect("Error loading posts");
            UserList(result)
    }
}



//impl UserList {
//    pub fn list(connection: &PgConnection) -> Self {
//        let result = users
//            .limit(10)
//            .load::<User>(connection)
//            .expect("Error loading users");
//        UserList(result)
//    }
//}
//