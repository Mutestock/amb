use warp::{
    filters::BoxedFilter,
    Filter,
};

pub fn health() -> BoxedFilter<()>{
    warp::get()
        .and(warp::path("health"))
        .boxed()
}

// DO NOT INCLUDE THIS ENDPOINT IN ANY SHAPE OR FORM IN PRODUCTION
pub fn check_conn_string() -> BoxedFilter<()>{
    warp::get()
        .and(warp::path("conn"))
        .boxed()
}

pub fn basic_connection() -> BoxedFilter<()>{
    warp::get()
        .and(warp::path("pgbasic"))
        .boxed()
}
