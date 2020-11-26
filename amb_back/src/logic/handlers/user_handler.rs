use warp;
use std::time::SystemTime;

use crate::{
    data_access::{
        entities::account::user::{
            UserList,
            User,
            NewUser,
            UserResponse
        },
        connection::pg_connection::POOL,
        pw_encryption,
    },
};



pub async fn list()-> Result<impl warp::Reply, warp::Rejection>{
    let conn = POOL.get().unwrap();
    let response = UserList::list(&conn);
    println!("{:#?}",&response);

    Ok(warp::reply::json(&response))
}

pub async fn login(username: String, password: String) -> Result<impl warp::Reply, warp::Rejection>{
    let conn = POOL.get().unwrap();
    let response = User::find_by_username(&username, &conn);
    let reply = match response {
        Ok(user) => {
            let matching_passwords = pw_encryption::verify_password(&user.username, &password, &user.salt.unwrap());
            if matching_passwords {
                UserResponse{
                    username: user.username,
                    email: user.email,
                    description: user.description,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    last_login: Some(SystemTime::now()),
                    admin: user.admin,
                }
            }
            else {
                // Custom error recommended
                return Err(warp::reject::not_found())
            }
        },
        Err(e)=> {
            println!("{:#?}",e);
            // Custom error recommended
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = User::find(&id, &conn);

    let reply = match response {
        Ok(user) =>{
            println!("{:#?}",&user);
            user
        },
        Err(e)=> {
            println!("{:#?}",e);
            // Custom error recommended
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}


pub async fn create(new_user: NewUser) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_user.create(&conn);

    let reply = match response {
        Ok(new_user) => {
            println!("{:#?}",&new_user);
        },
        Err(e) => {
            println!("{:#?}",&e);
            // Custom error recommended
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn update(id:i32, update_user: NewUser) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = User::update(&id, &update_user, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}",&null);
            null
        },
        Err(e) => {
            println!("{:#?}",e);
            // Custom error recommended
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn delete(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = User::delete(&id, &conn);

    let reply = match response {
        Ok(null) =>{
            println!("{:#?}",&null);
            null
        },
        Err(e) => {
            println!("{:#?}",e);
            // Custom error recommended
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}

