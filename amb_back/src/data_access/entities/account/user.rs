
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::PgConnection;
use diesel::ExpressionMethods;
use serde_derive::{Deserialize, Serialize};

use crate::schema::users;
use crate::schema::users::dsl;
use crate::schema::users::dsl::*;
use crate::data_access::auth;

use std::time::SystemTime;
use rand::Rng;


#[derive(Insertable, Deserialize, AsChangeset, PartialEq)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub description: Option<String>,
    pub admin: bool,
    pub salt: Option<String>,
}

// For login reception
#[derive(Deserialize)]
pub struct UserAuth {
    pub username: String, 
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String, 
}

impl NewUser {
    pub fn create(&self, connection: &PgConnection) -> Result<User, diesel::result::Error> {        
        let salted = rand::thread_rng().gen::<[u8; 32]>();
        let salted = format!("{:?}",salted);
        let with_encryption = NewUser{
            username: self.username.to_string(),
            password: auth::hash_and_salt(&self.password, &salted),
            email: self.email.to_string(),
            description: self.description.to_owned(),
            admin: self.admin,
            salt: Some(salted),
        };

        //self.password = auth::hash_and_salt(&self.password, &salted);
        //self.salt = Some(salted);
        diesel::insert_into(users::table)
            .values(with_encryption)
            .get_result(connection)
    }
}

// For creating a response to the frontend with non-essential values excluded.
#[derive(Deserialize, Serialize, Clone)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
    pub description: Option<String>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub last_login: Option<SystemTime>,
    pub admin: bool,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub description: Option<String>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub last_login: Option<SystemTime>,
    pub admin: bool,
    pub salt: Option<String>,
}

impl User {
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
 
    pub fn find_by_username(incoming_username: &str, connection: &PgConnection) -> Result<User, diesel::result::Error>{
        users::table.filter(username.eq(incoming_username)).first(connection)
    }

    pub fn refresh_last_login_by_username(incoming_username: &str, connection: &PgConnection) -> Result<(), diesel::result::Error>{
        diesel::update(users.filter(username.eq(incoming_username)))
            .set(last_login.eq(Some(SystemTime::now())))
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
