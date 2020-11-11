
//#[macro_use]
//use presentation::api::health;

use warp::{self,Filter, Rejection, Reply};

mod logic;
mod presentation;

use self::{
    presentation::{
        routes::{
            basic_routes,
        },
    },
    logic::{
        handlers::{
            health_handler
        }
    }
};

#[tokio::main]
async fn main() {
    let end_points = health!().with(warp::log("health"));

    println!("Started server at localhost:8000");
    warp::serve(end_points).run(([0, 0, 0, 0], 8000)).await;
}

