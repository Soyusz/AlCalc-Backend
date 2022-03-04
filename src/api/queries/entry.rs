use crate::api::DBPool;
use crate::model::entry::Entry;
use crate::services::entry::get_all_entries;
use crate::services::entry::get_unverified_entries;
use crate::services::entry::get_verified_entries;
use rocket::{get, routes, Route};
use rocket_contrib::json::Json;

#[get("/")]
fn get_all(conn: DBPool) -> Json<Vec<Entry>> {
    Json(get_all_entries(conn))
}

#[get("/verified")]
fn get_verified(conn: DBPool) -> Json<Vec<Entry>> {
    Json(get_verified_entries(conn))
}

#[get("/unverified")]
fn get_unverified(conn: DBPool) -> Json<Vec<Entry>> {
    Json(get_unverified_entries(conn))
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![get_all, get_verified, get_unverified]
}
