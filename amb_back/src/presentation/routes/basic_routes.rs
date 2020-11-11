use warp::{
    filters::BoxedFilter,
    Filter,
};

//pub async fn welcome_route() -> Result(impl Reply){
//    warp::get()
//        .boxed()
//}
fn path_prefix() -> BoxedFilter<()> {
    warp::path("hello")
        .boxed()
}

pub fn health() -> BoxedFilter<()>{
    warp::get()
        .and(warp::path("health"))
        .boxed()
}