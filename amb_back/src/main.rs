use warp::{self,Filter, Rejection, Reply};
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
            file_serving_routes,
            user_routes,
            image_routes,
        },
    },
    logic::{
        rejections::{
            file_rejection,
        },
        handlers::{
            health_handler,
            file_serving_handler,
            user_handler,
            image_handler,
        }
    }
};

//let connection = data_access::connection::pg_connection::POOL;

#[tokio::main]
async fn main() {

    // for automatic migrations
    let connection = data_access::connection::pg_connection::POOL.get().unwrap();
    embed_migrations!();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "content-type"])
        .allow_methods(vec!["POST", "GET", "PUT", "DELETE"]);
        
    let download_route = warp::path("files")
        .and(warp::fs::dir("/usr/resources/")
        .with(cors));

    let img_routes_compressed = list_images!()
        .or(get_image!())
        .or(create_image!())
        .or(update_image!())
        .or(delete_image!());
    

    let router = health!()
        .or(check_basic_connection!())
        //DETACH UNDERLYING ROUTE IN PRODUCTION
        //.or(check_conn_string!())
        .or(download_route)
        .or(list_users!())
        .or(get_user!())
        .or(create_user!())
        .or(update_user!()) 
        .or(delete_user!())
        .or(img_routes_compressed)
        .recover(file_rejection::handle_file_rejection);
    
    //let end = health!().with(warp::log("health"));

    println!("Started server at localhost:8000");
    warp::serve(router).run(([0, 0, 0, 0], 8000)).await;
}

