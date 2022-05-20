use crate::services::jwt_token as JwtTokenService;
use crate::types::token::SessionToken;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SessionTokenReturnType {
    pub token: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for SessionToken {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<SessionToken, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        let token = keys[0].to_string();

        match JwtTokenService::validate::<SessionToken>(token) {
            Ok(payload) => Outcome::Success(SessionToken {
                session_id: payload.session_id,
            }),
            Err(_) => Outcome::Failure((rocket::http::Status::new(401, "Unauthorized"), ())),
        }
    }
}
