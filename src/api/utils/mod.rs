use rocket::response::status;
use rocket_contrib::json::Json;

pub mod auth_user;
pub mod session;
pub mod structs;

pub type Response<T> = Result<Json<T>, status::BadRequest<&'static str>>;
