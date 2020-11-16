use warp;
use dotenv::dotenv;
use std::env;
use crate::data_access::connection::pg_connection::establish_connection;

pub async fn health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("OK")
}

// Testing purposes only. Route should be DETACHED in production!
pub async fn check_conn_string() -> Result<impl warp::Reply, warp::Rejection> {
    let db_url_one =  env::var("DATABASE_URL")
        .expect("No DATABASE_URL environment variable was found..."); 

    dotenv().ok();

    let db_url_two =  env::var("DATABASE_URL")
        .expect("No dotenv DATABASE_URL environment variable was found..."); 

    Ok(format!("Conn pre: {}, post conn: {}",db_url_one, db_url_two))
}

pub async fn basic_connection() -> Result<impl warp::Reply, warp::Rejection>{
    let connection_attempt = establish_connection();
    let boxed = Some(Box::new(connection_attempt));
    let mut res_str = String::from("");
    match boxed{
        Some(x) => res_str = "Some value and no crash occured".to_owned(),
        None => res_str = "None value and no crash occured".to_owned()
    }
    Ok(res_str)
}