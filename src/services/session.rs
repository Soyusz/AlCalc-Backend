use crate::api::utils::structs::LoginCred;
use crate::model::session::Session;
use crate::api::DBPool;
use uuid::Uuid;
use crate::db::session as SessionRepo;
use crate::db::user as UserRepo;
use crate::model::user::{NewUser, User};
use crate::services::email as EmailService;
use crate::services::image as ImageService;
use crate::services::jwt_token as JwtTokenService;
use crate::services::token as TokenService;
use crate::sql_types::UserRoles;
use crate::types::token::VerifyAccountPayload;

pub fn create_session(user_id: Uuid, conn: &DBPool) -> Result<Session,&'static str> {
    Ok(Session::create_session(user_id))
        .and_then(|s| SessionRepo::add_new(s, conn))
        // TODO email
}

pub fn is_authorized(session_id: Uuid, conn: &DBPool) -> Result<Session,&'static str> {
    match SessionRepo::get_by_id(session_id,conn) {
        Some(s) if s.authorized => Ok(s),
        // TODO check date
        _ => Err("Session not authorized!")
    }
}

pub fn authorize(session_id: Uuid, conn: &DBPool) -> Result<Session,&'static str> {
    SessionRepo::authorize(session_id, conn)
}
