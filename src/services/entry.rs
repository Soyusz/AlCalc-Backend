use uuid::Uuid;

use crate::api::DBPool;
use crate::db::entry::add_new;
use crate::db::entry::get_all;
use crate::db::entry::get_unverified;
use crate::db::entry::get_verified;
use crate::db::entry::verify;
use crate::db::entry::NewEntry;
use crate::model::entry::Entry;

pub fn get_all_entries(conn: DBPool) -> Vec<Entry> {
    get_all(&conn)
}

pub fn insert_entry(entry: NewEntry, conn: DBPool) -> Result<Entry, bool> {
    let res = add_new(entry, &conn);
    match res {
        Some(e) => Ok(e),
        None => Err(true),
    }
}

pub fn verify_entry(id: Uuid, state: bool, conn: DBPool) -> Result<Entry, bool> {
    let res = verify(id, state, &conn);
    match res {
        Some(s) => Ok(s),
        None => Err(true),
    }
}

pub fn get_verified_entries(conn: DBPool) -> Vec<Entry> {
    get_verified(&conn)
}

pub fn get_unverified_entries(conn: DBPool) -> Vec<Entry> {
    get_unverified(&conn)
}
