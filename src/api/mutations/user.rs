use crate::api::utils::{auth_user::AuthReturn, Response};
use crate::api::DBPool;
use crate::db::user::NewUser;
use crate::model::user::User as UserModel;
use crate::services::user::{self as UserService, LoginCred};
use rocket::response::status;
use rocket::{post, routes, Route};
use rocket_contrib::json::Json;

#[post("/", format = "application/json", data = "<new_user>", rank = 1)]
fn post_new(conn: DBPool, new_user: Json<NewUser>) -> Response<UserModel> {
    Ok(new_user.into_inner())
        .and_then(|user| UserService::insert_user(user, conn))
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}

#[post("/", rank = 2)]
fn post_new_invalid() -> status::BadRequest<()> {
    status::BadRequest(None)
}

#[post("/login", format = "application/json", data = "<login_cred>", rank = 1)]
fn login(conn: DBPool, login_cred: Json<LoginCred>) -> Response<AuthReturn> {
    Ok(())
        .map(|_| login_cred.into_inner())
        .and_then(|login_cred| UserService::login(login_cred, conn))
        .map(|token| Json(AuthReturn { token: token }))
        .map_err(|_| status::BadRequest(None))
}

#[post("/login", rank = 2)]
fn login_invalid() -> status::BadRequest<&'static str> {
    status::BadRequest(Some("Invalid payload"))
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![post_new, post_new_invalid, login, login_invalid]
}
