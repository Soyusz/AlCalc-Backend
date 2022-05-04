use crate::api::utils::{auth_user::Auth, Response};
use crate::api::DBPool;
use crate::model::entry::Entry;
use crate::services::entry as EntryService;
use crate::services::user as UserService;
use crate::sql_types::EntryLabel;
use rocket::response::status;
use rocket::{get, patch, routes, Route};
use rocket_contrib::json::Json;
use uuid::Uuid;

#[get("/")]
fn get_all(conn: DBPool) -> Json<Vec<Entry>> {
    Json(EntryService::get_all_entries(conn))
}

#[patch("/verified", format = "application/json", data = "<tags>")]
fn get_verified_tags(conn: DBPool, tags: Json<Vec<EntryLabel>>) -> Json<Vec<Entry>> {
    Json(EntryService::get_verified_entries_tags(
        conn,
        tags.into_inner(),
    ))
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

#[get("/<id_string>", rank = 3)]
fn get_by_id(id_string: String, conn: DBPool) -> Response<Entry> {
    Ok(id_string.as_str())
        .and_then(|id| Uuid::parse_str(id))
        .map_err(|_| "Invalid id")
        .and_then(|id| EntryService::get_by_id(id, &conn))
        .map(|r| Json(r))
        .map_err(|e| status::BadRequest(Some(e)))
}

pub fn get_routes() -> Vec<Route> {
    routes![
        get_verified_tags,
        get_verified,
        get_unverified,
        get_unverified_unauth,
        get_my,
        get_my_unauth,
        get_all,
        get_by_id
    ]
}
