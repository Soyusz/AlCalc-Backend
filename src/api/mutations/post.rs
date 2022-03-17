use crate::api::utils::{auth_user::Auth, Response};
use crate::api::DBPool;
use crate::db::post::NewPost;
use crate::model::post::Post;
use crate::services::post as PostService;
use rocket::response::status;
use rocket::{post, routes, Route};
use rocket_contrib::json::Json;

#[post("/", format = "application/json", data = "<new_post>", rank = 1)]
fn post_new(auth: Auth, new_post: Json<NewPost>, conn: DBPool) -> Response<Post> {
    Ok(new_post.into_inner())
        .and_then(|post| PostService::insert(post, auth.user_id, conn))
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
