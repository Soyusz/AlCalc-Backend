use crate::api::middlewares::auth_user::Auth;
use crate::api::DBPool;
use crate::db::entry::NewEntry;
use crate::model::entry::Entry;
use crate::services::entry::insert_entry;
use crate::services::entry::verify_entry;
use rocket::{post, put, routes, Route};
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/", format = "application/json", data = "<new_entry>")]
fn post_new(
    _auth: Auth,
    conn: DBPool,
    new_entry: Json<NewEntry>,
) -> Result<Json<Entry>, Json<bool>> {
    let entry = new_entry.into_inner();
    let res = insert_entry(entry, conn);
    match res {
        Ok(s) => Ok(Json(s)),
        Err(e) => Err(Json(e)),
    }
}

#[put("/<id_string>/accept")]
fn verify_accept(id_string: String, conn: DBPool) -> Result<Json<Entry>, Json<bool>> {
    let id_res = Uuid::parse_str(id_string.as_str());
    let id;
    match id_res {
        Ok(r) => {
            id = r;
        }
        Err(_) => return Err(Json(false)),
    }
    let res = verify_entry(id, true, conn);
    match res {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(Json(e)),
    }
}

#[put("/<id_string>/reject")]
fn verify_reject(id_string: String, conn: DBPool) -> Result<Json<Entry>, Json<bool>> {
    let id_res = Uuid::parse_str(id_string.as_str());
    let id;
    match id_res {
        Ok(r) => {
            id = r;
        }
        Err(_) => return Err(Json(false)),
    }
    let res = verify_entry(id, false, conn);
    match res {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(Json(e)),
    }
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![post_new, verify_accept, verify_reject]
}
