use crate::api::utils::Response;
use crate::api::DBPool;
use crate::model::post::Post;
use crate::model::user::User;
use crate::services::like as likeService;
use crate::services::session as SessionService;
use crate::types::token::SessionToken;
use rocket::response::status;
use rocket::{get, routes, Route};
use rocket_contrib::json::Json;
use uuid::Uuid;

#[get("/<id_string>")]
fn get_by_post(conn: DBPool, id_string: String) -> Response<Vec<User>> {
    Ok(id_string.as_str())
        .and_then(|id| Uuid::parse_str(id))
        .map_err(|_| status::BadRequest(None))
        .and_then(|id| match likeService::get_by_post(id, conn) {
            Ok(vect) => Ok(Json(vect)),
            Err(_) => Err(status::BadRequest(None)),
        })
}

#[get("/", rank = 1)]
fn get_by_user(session_token: SessionToken, conn: DBPool) -> Response<Vec<Post>> {
    SessionService::is_authorized(session_token.session_id, &conn)
        .and_then(|session| likeService::get_by_user(session.user_id, conn))
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}

#[get("/", rank = 2)]
fn get_by_user_unothorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![get_by_post, get_by_user, get_by_user_unothorized,]
}
