use crate::api::middlewares::auth_user::AuthReturn;
use crate::api::DBPool;
use crate::db::user::NewUser;
use crate::model::user::User as UserModel;
use crate::services::user::{self as UserService, LoginCred};
use rocket::response::status;
use rocket::{post, routes, Route};
use rocket_contrib::json::Json;

#[post("/", format = "application/json", data = "<new_user>", rank = 1)]
fn post_new(
    conn: DBPool,
    new_user: Json<NewUser>,
) -> Result<Json<UserModel>, status::BadRequest<&'static str>> {
    let user = new_user.into_inner();
    match UserService::insert_user(user, conn) {
        Ok(u) => Ok(Json(u)),
        Err(e) => Err(status::BadRequest(Some(e))),
    }
}

#[post("/", rank = 2)]
fn post_new_invalid() -> status::BadRequest<()> {
    status::BadRequest(None)
}

#[post("/login", format = "application/json", data = "<login_cred>", rank = 1)]
fn login(conn: DBPool, login_cred: Json<LoginCred>) -> Result<Json<AuthReturn>, Json<bool>> {
    match UserService::login(login_cred.into_inner(), conn) {
        Ok(res) => Ok(Json(AuthReturn { token: res })),
        Err(_) => Err(Json(false)),
    }
}
#[post("/login", rank = 2)]
fn login_invalid() -> status::BadRequest<&'static str> {
    status::BadRequest(Some("Invalid payload"))
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![post_new, post_new_invalid, login, login_invalid]
}
