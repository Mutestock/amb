use warp;
use std::time::SystemTime;

use crate::{
    data_access::{
        entities::account::user::{
            UserList,
            User,
            NewUser,
            UserResponse,
            UserAuth,
            LoginResponse,
        },
        connection::pg_connection::POOL,
        auth,
    },
    logic::rejections::error_handling,
};

pub async fn list()-> Result<impl warp::Reply, warp::Rejection>{
    let conn = POOL.get().unwrap();
    let response = UserList::list(&conn);
    println!("{:#?}",&response);

    Ok(warp::reply::json(&response))
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

// Receives Authentication user body, containing username and password.
// Returns either an UserResponse, which is a body containing only the necessary information for the frontend application or an error. 
pub async fn login(user_auth: UserAuth) -> Result<impl warp::Reply, warp::Rejection>{
    let conn = POOL.get().unwrap();
    let response = User::find_by_username(&user_auth.username, &conn);
    match response {
        Ok(user) => {
            let matching_passwords = auth::verify_password(&user_auth.username, &user_auth.password, &user.salt.unwrap());
            println!("{}", matching_passwords);
            if matching_passwords {
                User::refresh_last_login_by_username(&user.username, &conn)
                    .expect("Failed at persisting new login time to database");
                
                let user_response = UserResponse{
                    username: user.username,
                    email: user.email,
                    description: user.description,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    last_login: Some(SystemTime::now()),
                    admin: user.admin,
                };

                let token = auth::create_jwt(user_response)
                    .map_err(|e| warp::reject::custom(e))?;
                Ok(warp::reply::json(&LoginResponse { token }))
            }
            else {
                println!("Error in password matching");
                return Err(warp::reject::custom(error_handling::Error::IncorrectCredentialsError))
            }
        },
        Err(e)=> {
            println!("{:#?}",e);
            return Err(warp::reject::custom(error_handling::Error::IncorrectCredentialsError))
        }
    }
}