use crate::api::middlewares::auth_user::Auth;
use crate::api::DBPool;
use crate::model::entry::Entry;
use crate::services::entry as EntryService;
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

#[get("/unverified")]
fn get_unverified(conn: DBPool) -> Json<Vec<Entry>> {
    Json(EntryService::get_unverified_entries(conn))
}

#[get("/my")]
fn get_my(auth: Auth, conn: DBPool) -> Json<Vec<Entry>> {
    Json(EntryService::get_users_entries(auth.user_id, conn))
}

pub fn get_routes() -> Vec<Route> {
    routes![get_all, get_verified, get_unverified, get_my]
}
