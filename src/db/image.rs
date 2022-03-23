use uuid::Uuid;
use diesel::{self, PgConnection};
use crate::model::image::Image;
use crate::schema::images::dsl::images as all_images;
use diesel::prelude::*;
use crate::schema::images::id as image_id;
use crate::schema::images;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Image> {
    let vec = all_images
        .find(id)
        .load::<Image>(conn)
        .unwrap_or_else(|_| -> Vec<Image> { vec![] });
    if vec.len() == 0 {
        None
    } else {
        Some(vec[0].clone())
    }
}

pub fn add_new(image: Image, conn: &PgConnection) -> Option<Image> {
    let query_res: Result<Vec<Uuid>, _> = diesel::insert_into(images::table)
        .values(&image)
        .returning(image_id)
        .get_results(conn);

    match query_res {
        Ok(v) => {
            let new_image = get_by_id(v[0], conn).clone();
            match new_image {
                Some(s) => Some(s),
                None => None,
            }
        }
        Err(_) => None,
    }
}
