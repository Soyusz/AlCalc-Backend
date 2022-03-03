use crate::api::DBPool;
use crate::model::user::User;
use crate::services::user as UserService;
use rocket::get;
use rocket_contrib::json::Json;

#[get("/")]
pub fn get_all(conn: DBPool) -> Json<Vec<User>> {
    Json(UserService::get_all(conn))
}
