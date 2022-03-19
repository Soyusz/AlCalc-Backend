use crate::api::utils::{auth_user::Auth, Response};
use crate::api::DBPool;
use crate::model::entry::Entry;
use crate::services::entry as EntryService;
use crate::services::user as UserService;
use rocket::response::status;
use rocket::{get, routes, Route};
use rocket_contrib::json::Json;

#[get("/")]
fn get_all(conn: DBPool) -> Json<Vec<Entry>> {
    Json(EntryService::get_all_entries(conn))
}

#[get("/verified")]
fn get_verified(conn: DBPool) -> Json<Vec<Entry>> {
    Json(EntryService::get_verified_entries(conn))
}

#[get("/unverified", rank = 1)]
fn get_unverified(auth: Auth, conn: DBPool) -> Response<Vec<Entry>> {
    UserService::check_admin(auth.user_id, &conn)
        .map(|_| EntryService::get_unverified_entries(conn))
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}
#[get("/unverified", rank = 2)]
fn get_unverified_unauth() -> status::Unauthorized<&'static str> {
    status::Unauthorized(Some("Unauthorized"))
}

#[get("/my", rank = 1)]
fn get_my(auth: Auth, conn: DBPool) -> Json<Vec<Entry>> {
    Json(EntryService::get_user_entries(auth.user_id, conn))
}

#[get("/my", rank = 2)]
fn get_my_unauth() -> status::Unauthorized<&'static str> {
    status::Unauthorized(Some("Unauthorized"))
}

pub fn get_routes() -> Vec<Route> {
    routes![
        get_verified,
        get_unverified,
        get_unverified_unauth,
        get_my,
        get_my_unauth,
        get_all
    ]
}
