use crate::api::DBPool;
use crate::db::user::NewUser;
use crate::model::user::User as UserModel;
use crate::services::user as UserService;
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

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![post_new]
}
