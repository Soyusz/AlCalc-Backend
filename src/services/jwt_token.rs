use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
struct JwtToken<T> {
    exp: usize,
    payload: T,
}

pub fn create<T: Serialize>(payload: T) -> Result<String, &'static str> {
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::weeks(10))
        .expect("valid timestamp")
        .timestamp();
    let jwtToken = JwtToken {
        payload: payload,
        exp: expiration as usize,
    };
    let key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let header = Header::new(Algorithm::HS512);
    encode(&header, &jwtToken, &key).map_err(|_| "cannot create token")
}

pub fn validate<T: DeserializeOwned>(token: String) -> Result<T, &'static str> {
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    decode::<JwtToken<T>>(&token, &key, &Validation::new(Algorithm::HS512))
        .map(|decoded| decoded.claims.payload)
        .map_err(|_| "Invalid token")
}
