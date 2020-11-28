use warp;
use uuid::Uuid;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};
use std::convert::Infallible;
use futures::TryStreamExt;
use bytes::BufMut;

use crate::{
    data_access::{
        entities::account::image::{
            ImageList,
            Image,
            NewImage,
        },
        connection::pg_connection::POOL,
    },
};


pub async fn list()-> Result<impl warp::Reply, warp::Rejection>{
    let conn = POOL.get().unwrap();
    let response = ImageList::list(&conn);
    println!("{:#?}",&response);

    Ok(warp::reply::json(&response))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Image::find(&id, &conn);

    let reply = match response {
        Ok(image) =>{
            println!("{:#?}",&image);
            image
        },
        Err(e)=> {
            println!("{:#?}",e);
            // Custom error recommended
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}


pub async fn create(new_image: NewImage) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_image.create(&conn);

    let reply = match response {
        Ok(new_image) => {
            println!("{:#?}",&new_image);
        },
        Err(e) => {
            println!("{:#?}",&e);
            // Custom error recommended
            return Err(warp::reject::not_found())
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn update(id:i32, update_image: NewImage) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Image::update(&id, &update_image, &conn);

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
    let response = Image::delete(&id, &conn);

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
