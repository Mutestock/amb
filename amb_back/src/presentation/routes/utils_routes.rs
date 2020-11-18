use warp::{
    filters::BoxedFilter,
    Filter,
}
use std::env;

fn path_prefix() -> BoxedFilter<()>{
    warp::path("utils")
        .boxed()
}

// DO NOT INCLUDE THIS ENDPOINT IN ANY SHAPE OR FORM IN A RELEASE VERSION
