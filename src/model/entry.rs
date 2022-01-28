use serde::{Deserialize, Serialize};
use diesel::{self, Queryable};
use uuid::Uuid;
use crate::schema::entries;

#[derive(Clone,Deserialize,Serialize)]
pub struct NewEntry {
    pub name: String,
    pub price: f64,
    pub voltage: f64,
    pub volume: f64,
    pub photo: String,
}

#[derive(Clone,Deserialize,Serialize,Queryable,Insertable)]
#[table_name="entries"]
pub struct Entry {
    id: Uuid,
    name: String,
    price: f64,
    voltage: f64,
    volume: f64,
    verified: bool,
    photo: String
}


impl Entry {
    pub fn new(entry: NewEntry) -> Entry {
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
}
