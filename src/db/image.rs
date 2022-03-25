use uuid::Uuid;
use diesel::{self, PgConnection};
use crate::model::image::Image;
use crate::schema::images::dsl::images as all_images;
use diesel::prelude::*;
use crate::schema::images::id as image_id;
use crate::schema::images;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Image> {
    all_images
        .find(id)
        .first::<Image>(conn)
        .ok()
}

pub fn add_new(image: Image, conn: &PgConnection) -> Result<Image, &'static str> {
    diesel::insert_into(images::table)
        .values(&image)
        .returning(image_id)
        .get_results(conn)
        .map_err(|_| () )
        .and_then( |v| get_by_id(v[0], conn ).ok_or(()) )
        .map_err(|_| "Insert failed")
}
