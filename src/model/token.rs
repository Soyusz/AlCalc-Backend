use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use std::env;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TokenPayload {
    pub user_id: Uuid,
    exp: usize,
}

pub fn create(user_id: &Uuid) -> Result<String, &'static str> {
    let jwt_secret = env::var("ROCKET_PORT").unwrap();

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("valid timestamp")
        .timestamp();

    let payload = TokenPayload {
        user_id: *user_id,
        exp: expiration as usize,
    };

    let key = EncodingKey::from_secret(jwt_secret.as_bytes());

    let header = Header::new(Algorithm::HS512);

    encode(&header, &payload, &key).map_err(|_| "cannot create token")
}

pub fn validate(token: String) -> Result<TokenPayload, errors::Error> {
    let jwt_secret = env::var("ROCKET_PORT").unwrap();
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());

    let decoded = decode::<TokenPayload>(&token, &key, &Validation::new(Algorithm::HS512));

    match decoded {
        Ok(x) => Ok(x.claims),
        Err(err) => Err(err),
    }
}
