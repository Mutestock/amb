use warp::{
    Reply,
    Rejection,
    reply,
}

pub async fn database_creation-> Result<impl Reply, Rejection>{
    Ok("Initial database script executed. DATA MIGHT BE WIPED")
}