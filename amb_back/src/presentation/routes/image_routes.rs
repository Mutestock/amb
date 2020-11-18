use warp::{
    filters::BoxedFilter,
    path,
    Filter,
};

use crate::data_access::entities::account::image::NewImage;

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "image" / ..)
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

pub fn create() -> BoxedFilter<(NewImage,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

pub fn update() -> BoxedFilter<(i32, NewImage,)> {
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

pub fn get_image_by_user_id() -> BoxedFilter<(i32, )>{
    warp::get()
        .and(path_prefix())
        .and(warp::path::("usr="))
        .and(warp::path::param::<i32>())
        .boxed()
}
