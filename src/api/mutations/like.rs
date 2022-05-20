use crate::api::utils::{auth_user::Auth, Response};
use crate::api::DBPool;
use crate::model::like::Like;
use crate::services::like as LikeService;
use crate::services::session as SessionService;
use crate::types::token::SessionToken;
use rocket::response::status;
use rocket::{delete, post, routes, Route};
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/<id_string>", rank = 1)]
pub fn like(session_token: SessionToken, conn: DBPool, id_string: String) -> Response<Like> {
    let post_id =
        Uuid::parse_str(id_string.as_str()).map_err(|_| status::BadRequest(Some("Invalid id")))?;

    SessionService::is_authorized(session_token.session_id, &conn)
        .and_then(|session| LikeService::like(session.user_id, post_id, conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))
}

#[delete("/<id_string>", rank = 1)]
pub fn unlike(session_token: SessionToken, conn: DBPool, id_string: String) -> Response<Like> {
    let post_id =
        Uuid::parse_str(id_string.as_str()).map_err(|_| status::BadRequest(Some("Invalid id")))?;

    SessionService::is_authorized(session_token.session_id, &conn)
        .and_then(|session| LikeService::unlike(session.user_id, post_id, conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))
}

#[delete("/", rank = 2)]
pub fn unlike_unothorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

#[post("/", rank = 2)]
pub fn like_unothorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![unlike, like]
}
