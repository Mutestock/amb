use argon2::{self, Config};

pub fn hash_and_salt(password: &str, salt: &str) -> String{
    let config = Config::default();
    return argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
}  

pub fn verify_password(password: &str, incoming_password: &str, salt: &str) -> bool{
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
    return argon2::verify_encoded(&hash, incoming_password.as_bytes()).unwrap();
}

