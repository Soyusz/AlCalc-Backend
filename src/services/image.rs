use crate::api::DBPool;
use crate::model::image::Image;
use crate::db::image as ImageFactory;
use std::env;
use uuid::Uuid;
use base64;

pub fn base_to_blob(base_string: String) -> Result<Vec<u8>, &'static str>{
    base64::decode(base_string)
    .map_err(|_| "cannot convert string to blob")
}

pub fn create_from_blob(value: Vec<u8>, conn: &DBPool) -> Result<Image, &'static str> {
    Ok(Image::create_image(value))
    .and_then(|i|  ImageFactory::add_new(i, conn) )
    
}

pub fn create_from_base(base: String, conn: &DBPool) -> Result<Image, &'static str>{
    base_to_blob(base)
    .and_then(|b| create_from_blob(b, conn))
}

pub fn gen_link(image: Image) -> String {
    let be_url = env::var("BE_URL").unwrap();
    be_url + "/photo/" + &image.id.to_string().to_string()
}

pub fn get_link(id: Uuid, conn: &DBPool) -> Result<String, &'static str> {
    ImageFactory::get_by_id(id, conn)
    .ok_or("invalid id")
    .map(|i| gen_link(i) )
}

pub fn get_by_id(id: Uuid, conn: &DBPool) -> Result<Image, &'static str> {
    ImageFactory::get_by_id(id, conn)
    .ok_or("Invalid id")
}