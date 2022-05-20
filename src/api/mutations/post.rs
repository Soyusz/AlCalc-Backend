use crate::api::utils::Response;
use crate::api::DBPool;
use crate::model::post::{NewPost, Post};
use crate::services::post as PostService;
use crate::services::session as SessionService;
use crate::types::token::SessionToken;
use rocket::response::status;
use rocket::{post, routes, Route};
use rocket_contrib::json::Json;

#[post("/", format = "application/json", data = "<new_post>", rank = 1)]
fn post_new(session_token: SessionToken, new_post: Json<NewPost>, conn: DBPool) -> Response<Post> {
    SessionService::is_authorized(session_token.session_id, &conn)
        .and_then(|session| PostService::insert(new_post.into_inner(), session.user_id, conn))
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}

#[post("/", rank = 2)]
fn post_new_invalid() -> status::BadRequest<()> {
    status::BadRequest(None)
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![post_new, post_new_invalid]
}
