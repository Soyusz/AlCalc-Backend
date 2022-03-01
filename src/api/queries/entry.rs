use crate::api::DBPool;
use crate::model::entry::Entry;
use crate::services::entry::get_all_entries;
use crate::services::entry::get_unverified_entries;
use crate::services::entry::get_verified_entries;
use rocket::get;
use rocket_contrib::json::Json;

#[get("/")]
pub fn get_all(conn: DBPool) -> Json<Vec<Entry>> {
    Json(get_all_entries(conn))
}

#[get("/verified")]
pub fn get_verified(conn: DBPool) -> Json<Vec<Entry>> {
    Json(get_verified_entries(conn))
}

#[get("/unverified")]
pub fn get_unverified(conn: DBPool) -> Json<Vec<Entry>> {
    Json(get_unverified_entries(conn))
}
