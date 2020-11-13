use warp::{
    filters::BoxedFilter,
    Filter,
}
use std::env;

fn path_prefix() -> BoxedFilter<()>{
    warp::path("utils")
        .boxed()
}

pub fn database_creation() -> BoxedFilter<()>{
    warp::get()
        .and(path_prefix())
        .and(warp::header::exact(DATABASE_API_KEY, env::var(DATABASE_API_KEY)))
        .boxed()
}
