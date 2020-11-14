
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

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "content-type"])
        .allow_methods(vec!["POST", "GET", "UPDATE", "DELETE"]);
        
    let download_route = warp::path("files")
        .and(warp::fs::dir("/usr/resources/")
        .with(cors));

    let router = health!()
        .or(download_route)
        .recover(file_rejection::handle_file_rejection);
    
    //let end = health!().with(warp::log("health"));

    println!("Started server at localhost:8000");
    warp::serve(router).run(([0, 0, 0, 0], 8000)).await;
}

