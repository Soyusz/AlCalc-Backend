use crate::api::DBPool;
use crate::model::user::User;
use crate::services::user as UserService;
use rocket::{get, routes, Route};
use rocket_contrib::json::Json;

#[get("/")]
fn get_all(conn: DBPool) -> Json<Vec<User>> {
    Json(UserService::get_all(conn))
}

#[get("/admin", rank = 1)]
fn admin(_arg: User) -> &'static str {
    "admin"
}

#[get("/admin", rank = 2)]
fn admin_no_auth() -> &'static str {
    "not authorized"
}

#[get("/me")]
fn me(arg: User) -> Json<Vec<User>> {
    Json(vec![arg])
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![get_all, admin, admin_no_auth, me]
}
