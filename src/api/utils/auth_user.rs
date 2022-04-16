use crate::services::jwt_token as JwtTokenService;
use crate::types::token::AuthTokenPayload;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Auth {
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthReturn {
    pub token: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Auth, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        let token = keys[0].to_string();

        match JwtTokenService::validate::<AuthTokenPayload>(token) {
            Ok(payload) => Outcome::Success(Auth {
                user_id: payload.user_id,
            }),
            Err(_) => Outcome::Failure((rocket::http::Status::new(401, "Unauthorized"), ())),
        }
    }
}
