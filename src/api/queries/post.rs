use crate::api::utils::Response;
use crate::api::DBPool;
use crate::model::post::Post;
use crate::services::post as PostService;
use crate::services::session as SessionService;
use crate::types::token::SessionToken;
use rocket::response::status;
use rocket::{get, routes, Route};
use rocket_contrib::json::Json;
use uuid::Uuid;

#[get("/<id_string>", rank = 4)]
fn get_by_id(id_string: String, conn: DBPool) -> Response<Post> {
    Ok(id_string.as_str())
        .and_then(|id| Uuid::parse_str(id))
        .map_err(|_| status::BadRequest(None))
        .and_then(|id| PostService::get_by_id(id, &conn).ok_or(status::BadRequest(None)))
        .map(|r| Json(r))
}

#[get("/user/<id_string>", rank = 3)]
fn get_by_user(id_string: String, conn: DBPool) -> Response<Vec<Post>> {
    Ok(id_string.as_str())
        .and_then(|id| Uuid::parse_str(id))
        .map(|id| PostService::get_by_user(id, &conn))
        .map(|r| Json(r))
        .map_err(|_| status::BadRequest(None))
}

#[get("/", rank = 1)]
fn get_all(conn: DBPool) -> Response<Vec<Post>> {
    Ok(PostService::get_all(conn))
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}
#[get("/", rank = 2)]
fn get_all_unauthorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

#[get("/feed", rank = 1)]
fn get_feed(session_token: SessionToken, conn: DBPool) -> Response<Vec<Post>> {
    SessionService::is_authorized(session_token.session_id, &conn)
        .map(|session| PostService::get_feed(session.user_id, &conn))
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}
#[get("/feed", rank = 2)]
fn get_feed_unauthorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![
        get_all,
        get_all_unauthorized,
        get_by_id,
        get_by_user,
        get_feed,
        get_feed_unauthorized
    ]
}
