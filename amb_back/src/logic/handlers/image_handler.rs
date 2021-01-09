use bytes::BufMut;
use futures::TryStreamExt;
use uuid::Uuid;
use warp;
use warp::{
    multipart::{FormData, Part},
    Rejection, Reply,
};

use crate::data_access::{
    connection::pg_connection::POOL,
   
};
use crate::entities::account::image::{Image, ImageList, NewImage};

/*
This functionality has been excluded from the finals due to time constraints.
Will be used if/when project continue
*/

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = ImageList::list(&conn);
    println!("{:#?}", &response);

    Ok(warp::reply::json(&response))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Image::find(&id, &conn);

    let reply = match response {
        Ok(image) => {
            println!("{:#?}", &image);
            image
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn create(new_image: NewImage) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_image.create(&conn);

    let reply = match response {
        Ok(new_image) => {
            println!("{:#?}", &new_image);
        }
        Err(e) => {
            println!("{:#?}", &e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn update(id: i32, update_image: NewImage) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Image::update(&id, &update_image, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}", &null);
            null
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn delete(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Image::delete(&id, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}", &null);
            null
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    for p in parts {
        if p.name() == "file" {
            let content_type = p.content_type();
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "media/wav" => {
                        file_ending = ".wav";
                    }
                    "media/mp3" => {
                        file_ending = ".mp3";
                    }
                    "image/png" => {
                        file_ending = ".png";
                    }
                    "image/jpg" => {
                        file_ending = ".jpg";
                    }
                    v => {
                        eprintln!("Invalid file type found = {} ", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            let file_name = format!("./files/{}.{}", Uuid::new_v4().to_string(), file_ending);
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;
            println!("created file: {}", file_name);
        }
    }

    Ok("success")
}
