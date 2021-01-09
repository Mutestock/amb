use bytes::BufMut;
use futures::TryStreamExt;
use serde_json;
use std::{env, fs, str};
use uuid::Uuid;
use warp::{
    multipart::{FormData, Part},
    Rejection, Reply,
};

use crate::entities::{
    account::user::User,
    sound::track::{NewTrack, NewTrackReception, Track, TrackList},
};
use crate::{data_access::connection::pg_connection::POOL, logic::rejections::error_handling};

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = TrackList::list(&conn);
    println!("{:#?}", &response);

    Ok(warp::reply::json(&response))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Track::find(&id, &conn);

    let reply = match response {
        Ok(track) => {
            println!("{:#?}", &track);
            track
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn create(new_track: NewTrack) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_track.create(&conn);

    let reply = match response {
        Ok(new_track) => {
            println!("{:#?}", &new_track);
        }
        Err(e) => {
            println!("{:#?}", &e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn update(id: i32, update_track: NewTrack) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Track::update(&id, &update_track, &conn);

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
    let response = Track::delete(&id, &conn);

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

pub async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;
    let mut file_value: Vec<u8> = vec![];
    let mut file_ending = "";
    let mut resource_path: String = String::from("");
    let _uuid: String = Uuid::new_v4().to_string();

    for p in parts {
        if p.name() == "file" {
            let content_type = p.content_type();
            match content_type {
                Some(file_type) => {
                    match file_type {
                        "audio/wave" => {
                            file_ending = "wav";
                            println!("file ending set to {}", &file_ending);
                        }
                        "audio/mp3" => {
                            file_ending = "mp3";
                            println!("file ending set to {}", &file_ending);
                        }
                        "audio/ogg" => {
                            file_ending = "ogg";
                            println!("file ending set to {}", &file_ending);
                        }
                        "audio/mpeg" => {
                            file_ending = "mpeg";
                            println!("file ending set to {}", &file_ending);
                        }
                        "audio/wav" => {
                            file_ending = "wav";
                            println!("file ending set to {}", &file_ending);
                        }
                        v => {
                            eprintln!("Invalid mime type found = {}. Checking for filetype in filename...", v);
                            let file_name = p.filename();
                            match file_name {
                                Some(f_name) => match f_name.split(".").collect::<Vec<&str>>()[1] {
                                    "wav" => {
                                        file_ending = "wav";
                                    }
                                    "mp3" => {
                                        file_ending = "mp3";
                                    }
                                    "ogg" => {
                                        file_ending = "ogg";
                                    }
                                    "mpeg" => {
                                        file_ending = "mpeg";
                                    }
                                    z => {
                                        eprintln!("Invalid file type in filename found = {}. Upload failure", z);
                                        return Err(warp::reject::custom(
                                            error_handling::Error::InvalidFileTypeError,
                                        ));
                                    }
                                },
                                None => {
                                    eprintln!("No file name for uploaded file. Upload failure");
                                    return Err(warp::reject::custom(
                                        error_handling::Error::MissingFileNameError,
                                    ));
                                }
                            };
                        }
                    }
                }
                None => {
                    eprintln!("file type could not be determined. Upload failure");
                    return Err(warp::reject::custom(
                        error_handling::Error::FileTypeDeterminationError,
                    ));
                }
            }

            file_value = p
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

        // Only one
        //let resource_path = env::var("RESOURCE_PATH")
        //    .expect("RESOURCE_PATH environment variable must be set.");
        //
        //let file_name = format!("/{}/{}.{}", resource_path, Uuid::new_v4().to_string(), file_ending);
        //
        //tokio::fs::write(&file_name, file_value).await.map_err(|e| {
        //    eprint!("error writing file: {}", e);
        //    warp::reject::reject()
        //})?;
        //println!("created file: {}", file_name);
        } else if p.name() == "track" {
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

            let str_vals = match str::from_utf8(&value) {
                Ok(v) => v,
                Err(_) => return Err(warp::reject::reject()),
            };

            println!("String values extracted from form data = {}", &str_vals);

            let new_track_reception: NewTrackReception = serde_json::from_str(str_vals)
                .expect("Either form string isn't shaped in a json structure or a track object could not be constructed from it");

            let conn = POOL.get().unwrap();

            // Pathing will be utilizing the user's username
            let user = User::find(&new_track_reception.user_id, &conn);
            let username = match user {
                Ok(u) => u.username,
                Err(_) => return Err(warp::reject::not_found()),
            };

            let root_resource_path =
                env::var("RESOURCE_PATH").expect("RESOURCE_PATH environment variable must be set.");
            let p = format!("{}/{}", &root_resource_path, &username);
            resource_path = p;

            // Create database entry
            let new_track: NewTrack =
                new_track_reception.to_new_track(_uuid.clone(), resource_path.clone().to_string());
            let response = new_track.create(&conn);

            let directory_path_to_create = format!("{}/{}", &resource_path, &username);

            println!("{}", &directory_path_to_create);

            fs::create_dir_all(directory_path_to_create).expect("Could not create directory");

            match response {
                Ok(new_track) => {
                    println!("{:#?}", &new_track);
                }
                Err(e) => {
                    println!("{:#?}", &e);
                    // Custom error recommended
                    return Err(warp::reject::not_found());
                }
            };
        }
    }
    // File writing
    let file_name = format!("/{}/{}.{}", resource_path, _uuid, file_ending);
    println!("Writing file as: {}...", file_name);
    tokio::fs::write(&file_name, file_value)
        .await
        .map_err(|e| {
            eprint!("error writing file: {}", e);
            warp::reject::reject()
        })?;
    Ok("201")
}
