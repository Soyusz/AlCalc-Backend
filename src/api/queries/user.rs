use crate::api::middlewares::auth_user::Auth;
use crate::api::DBPool;
use crate::model::user::User as UserModel;
use crate::services::user as UserService;
use rocket::response::status;
use rocket::{get, routes, Route};
use rocket_contrib::json::Json;

#[get("/", rank = 1)]
fn get_all(arg: Auth, conn: DBPool) -> Result<Json<Vec<UserModel>>, status::BadRequest<String>> {
    match UserService::get_all(arg.user_id, conn) {
        Ok(r) => Ok(Json(r)),
        Err(_) => Err(status::BadRequest(None)),
    }
}
#[get("/", rank = 2)]
fn get_all_unauthorized() -> status::Unauthorized<String> {
    status::Unauthorized(Some("eee".to_string()))
}

#[get("/me", rank = 1)]
fn me(auth: Auth, conn: DBPool) -> Result<Json<UserModel>, status::BadRequest<&'static str>> {
    match UserService::get_user(auth.user_id, conn) {
        Some(user) => Ok(Json(user)),
        None => Err(status::BadRequest(Some("Invalid token"))),
    }
}
#[get("/me", rank = 2)]
fn me_unauthorized() -> status::Unauthorized<String> {
    status::Unauthorized(None)
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![get_all, get_all_unauthorized, me, me_unauthorized]
}
