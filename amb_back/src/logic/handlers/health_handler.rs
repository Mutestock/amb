use warp;

pub async fn health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("OK")
}