use crate::model::entry::{create_entry, Entry};
use crate::schema::entries;
use crate::schema::entries::dsl::entries as all_entries;
use crate::schema::entries::id as entry_id;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::entries::verified;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewEntry {
    pub name: String,
    pub price: f64,
    pub voltage: f64,
    pub volume: f64,
    pub photo: String,
}

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Entry> {
    all_entries
        .find::<Uuid>(id)
        .first::<Entry>(conn)
        .ok()
}

pub fn get_all(conn: &PgConnection) -> Vec<Entry> {
    all_entries
        .load::<Entry>(conn)
        .unwrap_or_else(|_| -> Vec<Entry> { vec![] })
}

pub fn verify(id: Uuid, state: Option<bool>, conn: &PgConnection) -> Result<Entry, &'static str> {
    diesel::update(all_entries.find(id))
        .set(verified.eq(state))
        .get_result::<Entry>(conn)
        .map_err(|_| "Query failed")
}

pub fn add_new(entry: NewEntry, user_id: Uuid, conn: &PgConnection) -> Result<Entry, &'static str> {
    let new_entry  = create_entry(entry, user_id);
    diesel::insert_into(entries::table)
        .values(&new_entry)
        .returning(entry_id)
        .get_results(conn)
        .map_err(|_| () )
        .and_then( |v| get_by_id(v[0], conn ).ok_or(()) )
        .map_err(|_| "Insert failed")
}

pub fn get_verified(conn: &PgConnection) -> Vec<Entry> {
    all_entries
        .filter(entries::verified.eq_all(true))
        .load::<Entry>(conn)
        .unwrap_or_else(|_| -> Vec<Entry> { vec![] })
}

pub fn get_unverified(conn: &PgConnection) -> Vec<Entry> {
    all_entries
        .filter(entries::verified.is_null())
        .load::<Entry>(conn)
        .unwrap_or_else(|_| -> Vec<Entry> { vec![] })
}

pub fn get_users(user_id: Uuid, conn: &PgConnection) -> Vec<Entry> {
    all_entries
        .filter(entries::user_id.eq(user_id))
        .load::<Entry>(conn)
        .unwrap_or_else(|_| -> Vec<Entry> { vec![] })
}
