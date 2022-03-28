use uuid::Uuid;
use crate::services::image as ImageService;
use crate::api::DBPool;
use crate::db::entry::NewEntry;
use crate::db::entry::{add_new, get_all, get_unverified, get_users, get_verified, verify};
use crate::model::entry::Entry;

pub fn get_all_entries(conn: DBPool) -> Vec<Entry> {
    get_all(&conn)
}

pub fn insert_entry(entry: NewEntry, user_id: Uuid, conn: DBPool) -> Result<Entry, &'static str> {
    ImageService::create_from_base(entry.photo, &conn)
        .map(|file| ImageService::gen_link(file)  )
        .map(|link| 
            NewEntry {
                name: entry.name,
                voltage: entry.voltage,
                price: entry.price,
                volume: entry.volume,
                photo: link
        })
        .and_then( |entry| add_new(entry, user_id, &conn) )
}

pub fn verify_entry(id: Uuid, state: bool, conn: DBPool) -> Result<Entry, &'static str> {
    verify(id, Some(state), &conn)
}

pub fn get_verified_entries(conn: DBPool) -> Vec<Entry> {
    get_verified(&conn)
}

pub fn get_unverified_entries(conn: DBPool) -> Vec<Entry> {
    get_unverified(&conn)
}

pub fn get_user_entries(user_id: Uuid, conn: DBPool) -> Vec<Entry> {
    get_users(user_id, &conn)
}
