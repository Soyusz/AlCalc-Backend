use crate::api::middlewares::auth_user::AuthReturn;
use crate::api::DBPool;
use crate::db::user::NewUser;
use crate::model::user::User as UserModel;
use crate::services::user::{self as UserService, LoginCred};
use rocket::{post, routes, Route};
use rocket_contrib::json::Json;

#[post("/", format = "application/json", data = "<new_user>")]
fn post_new(conn: DBPool, new_user: Json<NewUser>) -> Result<Json<UserModel>, Json<bool>> {
    let user = new_user.into_inner();
    let res = UserService::insert_user(user, conn);
    match res {
        Ok(s) => Ok(Json(s)),
        Err(e) => Err(Json(e)),
    }
}

#[post("/login", format = "application/json", data = "<login_cred>")]
fn login(conn: DBPool, login_cred: Json<LoginCred>) -> Result<Json<AuthReturn>, Json<bool>> {
    match UserService::login(login_cred.into_inner(), conn) {
        Ok(res) => Ok(Json(AuthReturn { token: res })),
        Err(_) => Err(Json(false)),
    }
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![post_new, login]
}
