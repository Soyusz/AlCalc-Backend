use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use uuid::Uuid;

const JWT_SECRET: &[u8] = b"hwehjkawehjke";

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TokenPayload {
    pub user_id: Uuid,
    exp: usize,
}

pub fn create(user_id: &Uuid) -> Result<String, errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("valid timestamp")
        .timestamp();

    let payload = TokenPayload {
        user_id: *user_id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &payload, &EncodingKey::from_secret(JWT_SECRET))
}

pub fn validate(token: String) -> Result<TokenPayload, errors::Error> {
    let decoded = decode::<TokenPayload>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    );

    match decoded {
        Ok(x) => Ok(x.claims),
        Err(err) => Err(err),
    }
}
