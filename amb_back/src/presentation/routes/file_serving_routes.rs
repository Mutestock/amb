use warp::{
    filters::BoxedFilter,
    Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("file")
        .boxed()
}

// Replace with database interaction
pub fn get_track(track_uuid: &str) -> BoxedFilter<(String, )> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::path::param::<String>())
        .boxed()
}