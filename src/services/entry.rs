use uuid::Uuid;

use crate::api::DBPool;
use crate::db::entry::NewEntry;
use crate::db::entry::{add_new, get_all, get_unverified, get_users, get_verified, verify};
use crate::model::entry::Entry;

pub fn get_all_entries(conn: DBPool) -> Vec<Entry> {
    get_all(&conn)
}

pub fn insert_entry(entry: NewEntry, user_id: Uuid, conn: DBPool) -> Result<Entry, bool> {
    let res = add_new(entry, user_id, &conn);
    match res {
        Some(e) => Ok(e),
        None => Err(true),
    }
}

pub fn verify_entry(id: Uuid, state: bool, conn: DBPool) -> Result<Entry, bool> {
    let res = verify(id, Option::Some(state), &conn);
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

pub fn get_users_entries(user_id: Uuid, conn: DBPool) -> Vec<Entry> {
    get_users(user_id, &conn)
}
