use serde::{Deserialize, Serialize};
use diesel::{self, Queryable};
use uuid::Uuid;
use crate::{schema::entries, db::entry::NewEntry};

#[derive(Clone,Deserialize,Serialize,Queryable,Insertable)]
#[table_name="entries"]
pub struct Entry {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub voltage: f64,
    pub volume: f64,
    pub verified: bool,
    pub photo: String
}

pub fn create_entry(entry: NewEntry) -> Entry {
    Entry {
        id: Uuid::new_v4(),
        name: entry.name,
        price: entry.price,
        voltage: entry.voltage,
        volume: entry.volume,
        verified: false,
        photo: entry.photo
    }
}
