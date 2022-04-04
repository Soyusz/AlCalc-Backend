use crate::model::entry::Entry;
use crate::schema::entries::{self, table as all_entries};
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Entry> {
    all_entries.find::<Uuid>(id).first::<Entry>(conn).ok()
}

pub fn get_all(conn: &PgConnection) -> Vec<Entry> {
    all_entries.load::<Entry>(conn).unwrap_or(vec![])
}

pub fn verify(id: Uuid, state: Option<bool>, conn: &PgConnection) -> Result<Entry, &'static str> {
    diesel::update(all_entries.find(id))
        .set(entries::verified.eq(state))
        .get_result::<Entry>(conn)
        .map_err(|_| "Update failed")
}

pub fn add_new(entry: Entry, user_id: Uuid, conn: &PgConnection) -> Result<Entry, &'static str> {
    diesel::insert_into(all_entries)
        .values(&entry)
        .get_result::<Entry>(conn)
        .map_err(|_| "Insert failed")
}

pub fn get_verified(conn: &PgConnection) -> Vec<Entry> {
    all_entries
        .filter(entries::verified.eq_all(true))
        .load::<Entry>(conn)
        .unwrap_or(vec![])
}

pub fn get_unverified(conn: &PgConnection) -> Vec<Entry> {
    all_entries
        .filter(entries::verified.is_null())
        .load::<Entry>(conn)
        .unwrap_or(vec![])
}

pub fn get_users(user_id: Uuid, conn: &PgConnection) -> Vec<Entry> {
    all_entries
        .filter(entries::user_id.eq(user_id))
        .load::<Entry>(conn)
        .unwrap_or(vec![])
}
