use crate::api::utils::Response;
use crate::api::DBPool;
use crate::model::user::User as UserModel;
use crate::services::session as SessionService;
use crate::services::user as UserService;
use crate::types::token::SessionToken;
use rocket::response::status;
use rocket::{get, routes, Route};
use rocket_contrib::json::Json;
use uuid::Uuid;

#[get("/<id_string>", rank = 3)]
fn get_by_id(id_string: String, conn: DBPool) -> Response<UserModel> {
    Ok(id_string.as_str())
        .and_then(|id| Uuid::parse_str(id))
        .map_err(|_| status::BadRequest(None))
        .and_then(|id| UserService::get_user(id, &conn).ok_or(status::BadRequest(None)))
        .map(|r| Json(r))
}

#[get("/", rank = 1)]
fn get_all(session_token: SessionToken, conn: DBPool) -> Response<Vec<UserModel>> {
    SessionService::is_authorized(session_token.session_id, &conn)
        .and_then(|session| UserService::get_all(session.user_id, conn))
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}

#[get("/", rank = 2)]
fn get_all_unauthorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

#[get("/me", rank = 1)]
fn me(session_token: SessionToken, conn: DBPool) -> Response<UserModel> {
    SessionService::is_authorized(session_token.session_id, &conn)
        .and_then(|session| UserService::get_user(session.user_id, &conn).ok_or("twoja matka"))
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}

#[get("/me", rank = 2)]
fn me_unauthorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![
        get_all,
        get_all_unauthorized,
        me,
        me_unauthorized,
        get_by_id
    ]
}
