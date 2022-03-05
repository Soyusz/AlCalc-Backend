use jsonwebtoken::{decode, encode, errors, DecodingKey, EncodingKey, Header, Validation};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TokenPayload {
    user_id: String,
}

pub fn create(user_id: &String) -> Result<String, errors::Error> {
    let SECRET_KEY: u8 = std::env::var("JWT_SECRET").unwrap().parse::<u8>().unwrap();

    let payload = TokenPayload {
        user_id: user_id.to_string(),
    };

    encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(&[SECRET_KEY]),
    )
}

pub fn validate(token: String) -> Result<TokenPayload, errors::Error> {
    let SECRET_KEY: u8 = std::env::var("JWT_SECRET").unwrap().parse::<u8>().unwrap();

    let res = decode::<TokenPayload>(
        &token,
        &DecodingKey::from_secret(&[SECRET_KEY]),
        &Validation::default(),
    )
    .unwrap();

    Result::Ok(res.claims)
}
