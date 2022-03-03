use crate::api::DBPool;
use crate::db::user::NewUser;
use crate::model::user::User as UserModel;
use crate::services::user as UserService;
use rocket::post;
use rocket_contrib::json::Json;

#[post("/", format = "application/json", data = "<new_user>")]
pub fn post_new(conn: DBPool, new_user: Json<NewUser>) -> Result<Json<UserModel>, Json<bool>> {
    let user = new_user.into_inner();
    let res = UserService::insert_user(user, conn);
    match res {
        Ok(s) => Ok(Json(s)),
        Err(e) => Err(Json(e)),
    }
}
