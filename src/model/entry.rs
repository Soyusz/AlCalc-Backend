use crate::model::user::User;
use crate::sql_types::EntryLabel;
use crate::{db::entry::NewEntry, schema::entries};
use diesel::{self, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Queryable, Insertable, Associations)]
#[belongs_to(User)]
#[table_name = "entries"]
pub struct Entry {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub voltage: f64,
    pub volume: f64,
    pub verified: Option<bool>,
    pub photo: String,
    pub user_id: Uuid,
    pub label: Vec<EntryLabel>,
}

pub fn create_entry(entry: NewEntry, user_id: Uuid) -> Entry {
    Entry {
        id: Uuid::new_v4(),
        name: entry.name,
        price: entry.price,
        voltage: entry.voltage,
        volume: entry.volume,
        verified: None,
        photo: entry.photo,
        user_id: user_id,
        label: entry.label
    }
}
