use crate::api::utils::{auth_user::Auth, Response};
use crate::api::DBPool;
use crate::model::entry::{Entry, NewEntry};
use crate::services::entry as EntryService;
use crate::services::session as SessionService;
use crate::types::token::SessionToken;
use rocket::response::status;
use rocket::{post, put, routes, Route};
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/", format = "application/json", data = "<new_entry>", rank = 1)]
fn post_new(
    session_token: SessionToken,
    conn: DBPool,
    new_entry: Json<NewEntry>,
) -> Response<Entry> {
    SessionService::is_authorized(session_token.session_id, &conn)
        .and_then(|session| {
            EntryService::insert_entry(new_entry.into_inner(), session.user_id, conn)
        })
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
        .and_then(|id| EntryService::verify_entry(id, true, conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))
}

#[put("/<id_string>/reject")]
fn verify_reject(id_string: String, conn: DBPool) -> Response<Entry> {
    Uuid::parse_str(id_string.as_str())
        .map_err(|_| "cannot parse id")
        .and_then(|id| EntryService::verify_entry(id, false, conn))
        .map_err(|e| status::BadRequest(Some(e)))
        .map(|r| Json(r))
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![post_new, verify_accept, verify_reject, post_new_invalid]
}
