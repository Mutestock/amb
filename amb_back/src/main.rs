
//#[macro_use]
//use presentation::api::health;

use warp::{self,Filter, Rejection, Reply};

mod logic;
mod presentation;

use self::{
    presentation::{
        routes::{
            basic_routes,
            file_serving_routes,
        },
    },
    logic::{
        rejections::{
            file_rejection,
        },
        handlers::{
            health_handler,
            file_serving_handler,
        }
    }
};

#[tokio::main]
async fn main() {

    let download_route = warp::path("files").and(warp::fs::dir("../resources"));

    let router = health!()
        .or(download_route)
        .recover(file_rejection::handle_file_rejection);
    
    //let end = health!().with(warp::log("health"));

    println!("Started server at localhost:8000");
    warp::serve(router).run(([0, 0, 0, 0], 8000)).await;
}

