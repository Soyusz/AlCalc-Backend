use crate::api::DBPool;
use crate::model::user::User;
use crate::services::user as UserService;
use rocket::request::{self, FromRequest, Request};
use rocket::{get, routes, Outcome, Route};
use rocket_contrib::json::Json;

#[get("/")]
fn get_all(conn: DBPool) -> Json<Vec<User>> {
    Json(UserService::get_all(conn))
}

pub struct ApiKey(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        let key = ApiKey(keys[0].to_string());
        Outcome::Success(key)
    }
}

#[get("/admin", rank = 1)]
fn admin(_arg: ApiKey) -> &'static str {
    "admin access only"
}

#[get("/admin", rank = 2)]
fn admin_no_auth() -> &'static str {
    "not authorized"
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![get_all, admin, admin_no_auth]
}
