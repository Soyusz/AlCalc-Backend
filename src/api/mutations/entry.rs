use crate::api::utils::{auth_user::Auth, Response};
use crate::api::DBPool;
use crate::db::entry::NewEntry;
use crate::model::entry::Entry;
use crate::services::entry::insert_entry;
use crate::services::entry::verify_entry;
use rocket::response::status;
use rocket::{post, put, routes, Route};
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/", format = "application/json", data = "<new_entry>", rank = 1)]
fn post_new(auth: Auth, conn: DBPool, new_entry: Json<NewEntry>) -> Response<Entry> {
    Ok(new_entry.into_inner())
        .and_then(|entry| insert_entry(entry, auth.user_id, conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))
}
#[post("/", rank = 2)]
fn post_new_invalid() -> status::BadRequest<&'static str> {
    status::BadRequest(Some("Invalid payload"))
}

#[put("/<id_string>/accept")]
fn verify_accept(id_string: String, conn: DBPool) -> Response<Entry> {
    Uuid::parse_str(id_string.as_str())
        .map_err(|_| "cannot parse id")
        .and_then(|id| verify_entry(id, true, conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))
}

#[put("/<id_string>/reject")]
fn verify_reject(id_string: String, conn: DBPool) -> Response<Entry> {
    Uuid::parse_str(id_string.as_str())
        .map_err(|_| "cannot parse id")
        .and_then(|id| verify_entry(id, false, conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![post_new, verify_accept, verify_reject, post_new_invalid]
}
