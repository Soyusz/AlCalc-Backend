use crate::api::DBPool;
use crate::db::image as ImageFactory;
use crate::model::image::Image;
use base64;
use data_url::DataUrl;
use std::env;
use uuid::Uuid;

pub fn base_to_blob(base_string: String) -> Result<Vec<u8>, &'static str> {
    base64::decode(base_string).map_err(|_| "cannot convert string to blob")
}

pub fn create_from_blob(value: Vec<u8>, conn: &DBPool) -> Result<Image, &'static str> {
    Ok(Image::create_image(value)).and_then(|i| ImageFactory::add_new(i, conn))
}

pub fn create_from_base(base: String, conn: &DBPool) -> Result<Image, &'static str> {
    DataUrl::process(base.as_str())
        .map_err(|_| "Error processing data-url")
        .and_then(|url| url.decode_to_vec().map_err(|_| "Error decoding to vector"))
        .and_then(|body| create_from_blob(body.0, conn))
}

pub fn gen_link(image: Image) -> String {
    let be_url = env::var("BE_URL").unwrap();
    be_url + "/photo/" + &image.id.to_string().to_string()
}

pub fn get_link(id: Uuid, conn: &DBPool) -> Result<String, &'static str> {
    ImageFactory::get_by_id(id, conn)
        .ok_or("invalid id")
        .map(|i| gen_link(i))
}

pub fn get_by_id(id: Uuid, conn: &DBPool) -> Result<Image, &'static str> {
    ImageFactory::get_by_id(id, conn).ok_or("Invalid id")
}
