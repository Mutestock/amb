use argon2::{self, Config};
use chrono::Utc;
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

use crate::entities::account::user::UserResponse;
use crate::logic::rejections::error_handling::Error;

const BEARER: &str = "Bearer ";

fn get_jwt_secret() -> String {
    env::var("AMBIENCE_JWT_TOKEN_SECRET")
        .expect("Missing AMBIENCE_JWT_TOKEN_SECRET environment variable")
}

// Header
// Payload
// Signature

#[derive(Serialize, Deserialize)]
struct Claims {
    user_response: UserResponse,
    exp: usize,
}

pub fn hash_and_salt(password: &str, salt: &str) -> String {
    let config = Config::default();
    return argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
}

pub fn verify_password(password: &str, incoming_password: &str, salt: &str) -> bool {
    let config = Config::default();
    let hash =
        argon2::hash_encoded(incoming_password.as_bytes(), salt.as_bytes(), &config).unwrap();
    return hash == password;
    //return argon2::verify_encoded(&hash, password.as_bytes()).unwrap();
}

pub fn create_jwt(user_response: UserResponse) -> Result<String, Error> {
    dotenv().ok();

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(24))
        .expect("Some valid timestamp")
        .timestamp();

    let claims = Claims {
        user_response,
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes()),
    )
    .map_err(|_| Error::JWTTokenCreationError)
}

pub fn with_auth(
    user_response: UserResponse,
) -> impl Filter<Extract = (UserResponse,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (user_response.clone(), headers))
        .and_then(authorize)
}

async fn authorize(
    (user_response, headers): (UserResponse, HeaderMap<HeaderValue>),
) -> Result<UserResponse, Rejection> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::InvalidToken))?;

            if &user_response.admin != &decoded.claims.user_response.admin {
                return Err(reject::custom(Error::NoPermissionError));
            }
            Ok(decoded.claims.user_response)
        }
        Err(e) => return Err(reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String, Error> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(Error::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}

#[test]
fn test_hash_and_salt() { 
    let test_password = "Some Password";
    let test_salt = "Some Salt";
    let hashed_and_salted = hash_and_salt(&test_password, &test_salt);
    
    assert_eq!(verify_password(&hashed_and_salted, &test_password,  &test_salt), true);
}