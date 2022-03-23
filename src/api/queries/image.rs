use crate::api::DBPool;
use rocket::{get, routes, Route};
use crate::model::image::Image;
use crate::services::image as ImageService;
use uuid::Uuid;
use rocket_contrib::json::Json;
use rocket::response::status;

#[get("/<id_string>")]
fn get_by_id(
    conn: DBPool,
    id_string: String
) -> Result<Vec<u8>, status::BadRequest<&'static str>> {
    Ok(id_string.as_str())
        .and_then(|id| Uuid::parse_str(id))
        .map_err(|_| "Invalid id")
        .and_then(|id| ImageService::get_by_id(id, &conn) )
        .map(|image| image.value)
        .map_err(|e| status::BadRequest(Some(e)))
}

pub fn get_routes() -> std::vec::Vec<Route> {
    routes![
        get_by_id
    ]
}
