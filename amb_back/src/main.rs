use warp::{self,Filter};
extern crate chrono;
extern crate argon2;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod logic;
mod presentation;
mod data_access;
mod schema;

use self::{
    presentation::{
        routes::{
            basic_routes,
            user_routes,
            image_routes,
            track_routes,
        },
    },
    logic::{
        rejections::{
            error_handling,
        },
        handlers::{
            basic_handler,
            user_handler,
            image_handler,
            track_handler,
        }
    }
};

//let connection = data_access::connection::pg_connection::POOL;

#[tokio::main]
async fn main() {
    // Edgy message to signal when the main has been initiated in the docker process.
    println!("Hush...");

    // for automatic migrations
    let connection = data_access::connection::pg_connection::POOL.get().unwrap();
    embed_migrations!();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Diesel embedded migrations failed!");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "content-type","Access-Control-Allow-Origin"])
        .allow_methods(vec!["POST", "GET", "PUT", "DELETE"])
        .build();
    
    let log = warp::log("api::request");
        
    let download_route = warp::path("files")
        .and(warp::fs::dir("/usr/resources/"));

    let image_routes = list_images!()
        .or(get_image!())
        .or(create_image!())
        .or(update_image!())
        .or(delete_image!());
    
    let track_routes = upload_track!()
        .or(list_tracks!())
        .or(get_track!())
        .or(create_track!())
        .or(update_track!())
        .or(delete_track!());

    let user_routes = list_users!()
        .or(get_user!())
        .or(create_user!())
        .or(update_user!())
        .or(delete_user!())
        .or(login_user!());
        
    let router = health!()
        .or(check_basic_connection!())
        //DETACH UNDERLYING ROUTE IN PRODUCTION
        //.or(check_conn_string!())
        .or(download_route)
        .or(user_routes)
        .or(image_routes)
        .or(track_routes)
        .or(home!())
        .recover(error_handling::handle_rejection)
        .with(log)
        .with(cors);
    
    println!("Started server at localhost:8000");
    warp::serve(router).run(([0, 0, 0, 0], 8000)).await;
}

