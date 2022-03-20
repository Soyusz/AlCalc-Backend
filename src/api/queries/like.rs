use crate::api::utils::{auth_user::Auth, Response};
use crate::api::DBPool;
use rocket::{get, routes, Route};
use crate::model::user::User;
use crate::model::post::Post;
use crate::services::like as likeService;
use uuid::Uuid;
use rocket_contrib::json::Json;
use rocket::response::status;

#[get("/<id_string>")]
fn get_by_post(
    conn: DBPool,
    id_string: String
) -> Response<Vec<User>> {
    Ok(id_string.as_str())
        .and_then(|id| Uuid::parse_str(id))
        .map_err(|_| status::BadRequest(None))
        .and_then(|id| {
            match likeService::get_by_post(id, conn){
                Ok(vect) => Ok(Json(vect)),
                Err(_) => Err(status::BadRequest(None))
            }
        })
}

#[get("/", rank=1)]
fn get_by_user(
    auth: Auth,
    conn: DBPool,
) -> Response<Vec<Post>> {
    match likeService::get_by_user(auth.user_id, conn) {
        Ok(vect) => Ok(Json(vect)),
        Err(_) => Err(status::BadRequest(None))
    }
}

#[get("/", rank = 2)]
fn get_by_user_unothorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![
        get_by_post,
        get_by_user,
        get_by_user_unothorized,
    ]
}