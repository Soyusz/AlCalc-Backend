use crate::model::image::Image;
use crate::schema::images::table as all_images;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Image> {
    all_images.find(id).first::<Image>(conn).ok()
}

pub fn add_new(image: Image, conn: &PgConnection) -> Result<Image, &'static str> {
    diesel::insert_into(all_images)
        .values(&image)
        .get_result(conn)
        .map_err(|_| "Insert failed")
}
