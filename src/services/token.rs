use crate::services::jwt_token as JwtTokenService;
use crate::types::token::{AuthTokenPayload, VerifyAccountPayload};
use uuid::Uuid;

pub fn create_auth_token(user_id: &Uuid) -> Result<String, &'static str> {
    Ok(AuthTokenPayload { user_id: *user_id }).and_then(|payload| JwtTokenService::create(payload))
}

pub fn create_account_verification_token(user_id: &Uuid) -> Result<String, &'static str> {
    Ok(VerifyAccountPayload { user_id: *user_id })
        .and_then(|payload| JwtTokenService::create(payload))
}
