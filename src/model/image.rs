use crate::schema::images;
use diesel::{self, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Queryable, Insertable, Associations, Debug)]
#[table_name = "images"]
pub struct Image {
    pub id: Uuid,
    pub value: Vec<u8>,
}

impl Image {
    pub fn create_image(blob: Vec<u8>) -> Image {
        Image {
            id: Uuid::new_v4(),
            value: blob,
        }
    }
}
