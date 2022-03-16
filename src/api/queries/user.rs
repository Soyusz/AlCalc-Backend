use crate::api::utils::{auth_user::Auth, Response};
use crate::api::DBPool;
use crate::model::user::User as UserModel;
use crate::services::user as UserService;
use rocket::response::status;
use rocket::{get, routes, Route};
use rocket_contrib::json::Json;

#[get("/", rank = 1)]
fn get_all(auth: Auth, conn: DBPool) -> Response<Vec<UserModel>> {
    UserService::get_all(auth.user_id, conn)
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}
#[get("/", rank = 2)]
fn get_all_unauthorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

#[get("/me", rank = 1)]
fn me(auth: Auth, conn: DBPool) -> Response<UserModel> {
    match UserService::get_user(auth.user_id, conn) {
        Some(user) => Ok(Json(user)),
        None => Err(status::BadRequest(Some("Invalid token"))),
    }
}
#[get("/me", rank = 2)]
fn me_unauthorized() -> status::Unauthorized<()> {
    status::Unauthorized(None)
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![get_all, get_all_unauthorized, me, me_unauthorized]
}
