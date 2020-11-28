use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use argon2::{self, Config};
use dotenv::dotenv;
use std::env;

use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

// Header
// Payload
// Signature


pub fn hash_and_salt(password: &str, salt: &str) -> String{
    let config = Config::default();
    return argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
}  

pub fn verify_password(password: &str, incoming_password: &str, salt: &str) -> bool{
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
    return argon2::verify_encoded(&hash, incoming_password.as_bytes()).unwrap();
}

pub fn create_jwt(uid: &str, admin: &bool) -> Result<String> {
    dotenv().ok();

    const SECRET = env::var("AMBIENCE_JWT_TOKEN_SECRET")
        .expect("Missing AMBIENCE_JWT_TOKEN_SECRET environment variable");

    let experiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(24))
        .expect("Some valid timestamp")
        .timestamp();
    
    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(SECRET))
        .map_err(|_| Error::JWTTokenCreationError)
}

pub fn with_auth(is_admin: bool) -> impl Filter<extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: Headermap<HeaderValue>| (is_admin.clone(), headers))
        .and_then(authorize)
}

async fn authorize((is_admin, headers): (Role, HeaderMap<HeaderValue>)) -> Result<String, error::Error> {
    match jwt_from_headers(&headers) {
        Ok(jwt) => {
            // Insert stuff
            https://blog.logrocket.com/jwt-authentication-in-rust/
        }
        Err(e) => return Err(reject::custom(e)),
    }
}


