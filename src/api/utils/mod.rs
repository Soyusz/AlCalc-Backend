use rocket::response::status;
use rocket_contrib::json::Json;

pub mod auth_user;
pub type Response<T> = Result<Json<T>, status::BadRequest<&'static str>>;
