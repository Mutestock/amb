use warp::{
    filters::BoxedFilter,
    Filter,
    path
};

use crate::data_access::entities::sound::track::{NewTrack};

// Courtesy of steadylearner
// https://github.com/steadylearner/Rust-Full-Stack/blob/master/warp/database/2.%20with_db_pool/src/routes/post_route_without_reusable.rs


fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "track" / ..)
        .boxed()
}

pub fn list() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .boxed()
}

pub fn get() -> BoxedFilter<(i32, )> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .boxed()
}

pub fn create() -> BoxedFilter<(NewTrack,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

pub fn update() -> BoxedFilter<(i32, NewTrack,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::put()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .and(json_body)
        .boxed()
}

pub fn delete() -> BoxedFilter<(i32, )> {
    warp::delete()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .boxed()
}
// Consider uuid param
pub fn upload() -> BoxedFilter<(warp::multipart::FormData, )> {
    warp::post()
        .and(path_prefix())
        .and(warp::path("upload"))
        .and(warp::multipart::form().max_length(5_000_000))
        .boxed()
}