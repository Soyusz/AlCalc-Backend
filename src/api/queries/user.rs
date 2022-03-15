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

#[get("/admin", rank = 1)]
fn admin(_arg: Auth) -> &'static str {
    "admin"
}

#[get("/admin", rank = 2)]
fn admin_no_auth() -> &'static str {
    "not authorized"
}

#[get("/me")]
fn me(auth: Auth, conn: DBPool) -> Result<Json<UserModel>, &'static str> {
    match UserService::get_user(auth.user_id, conn) {
        Some(user) => Ok(Json(user)),
        None => Err("invalid token"),
    }
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![get_all, get_all_unauthorized, admin, admin_no_auth, me]
}
