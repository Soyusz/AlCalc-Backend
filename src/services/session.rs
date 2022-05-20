use crate::api::utils::structs::LoginCred;
use crate::model::session::Session;
use crate::api::DBPool;
use uuid::Uuid;
use crate::db::session as SessionRepo;
use crate::services::user as UserService;
use crate::db::user as UserRepo;
use crate::model::user::{NewUser, User};
use crate::services::email as EmailService;
use crate::services::image as ImageService;
use crate::services::jwt_token as JwtTokenService;
use crate::services::token as TokenService;
use crate::sql_types::UserRoles;
use crate::types::token::SessionToken;

pub fn create_session(user_id: Uuid, conn: &DBPool) -> Result<Session,&'static str> {
    Ok(Session::create_session(user_id))
        .and_then(|s| SessionRepo::add_new(s, conn))
}

pub fn is_authorized(session_id: Uuid, conn: &DBPool) -> Result<Session,&'static str> {
    match SessionRepo::get_by_id(session_id,conn) {
        Some(s) if s.authorized => Ok(s),
        // TODO check date
        _ => Err("Session not authorized!")
    }
}

pub fn authorize(token: String, conn: &DBPool) -> Result<Session,&'static str> {
    JwtTokenService::validate::<SessionToken>(token)
        .and_then(|r| SessionRepo::authorize(r.session_id, conn))
}

pub fn login(login_cred: LoginCred, conn: &DBPool) -> Result<String, &'static str> {
    UserService::get_by_email(login_cred.email.clone(), conn)
       .ok_or("Cannot log in")
       .and_then(|u| create_session(u.id, conn))
       .and_then(|s| {
           EmailService::session_verification(login_cred.email,s.id)?;
           Ok(s)
       })
        .and_then(|session| TokenService::create_session_token(&session.id) )
}

