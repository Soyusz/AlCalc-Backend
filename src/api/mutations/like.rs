use crate::api::utils::{auth_user::Auth, Response};
use crate::api::DBPool;
use rocket::{post,delete, routes, Route};
use crate::model::like::Like;
use crate::services::like as likeService;
use uuid::Uuid;
use rocket_contrib::json::Json;
use rocket::response::status;

#[post("/<id_string>", rank=1)]
pub fn like(
    auth: Auth,
    conn: DBPool,
    id_string: String
) -> Response<Like> {
    Ok(id_string.as_str())
        .and_then(|id| Uuid::parse_str(id))
        .map_err(|_| "Invalid id")
        .and_then(|id| likeService::like(auth.user_id, id,conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))

}

#[delete("/<id_string>", rank=1)]
pub fn unlike(
    auth: Auth,
    conn: DBPool,
    id_string: String
) -> Response<()> {
    Ok(id_string.as_str())
        .and_then(|id| Uuid::parse_str(id))
        .map_err(|_| "Invalid id")
        .and_then(|id| likeService::unlike(auth.user_id, id,conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))
}

#[delete("/", rank=2)]
pub fn unlike_unothorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

#[post("/", rank=2)]
pub fn like_unothorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![
        unlike,
        like
    ]
}