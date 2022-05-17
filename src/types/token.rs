use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthTokenPayload {
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VerifyAccountPayload {
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthorizeSessionPayload {
    pub session_id: Uuid,
}
