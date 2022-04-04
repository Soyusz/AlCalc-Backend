use crate::api::DBPool;
use crate::db::entry as EntryRepo;
use crate::model::entry::{Entry, NewEntry};
use crate::services::image as ImageService;
use uuid::Uuid;

pub fn get_all_entries(conn: DBPool) -> Vec<Entry> {
    EntryRepo::get_all(&conn)
}

pub fn insert_entry(
    raw_entry: NewEntry,
    user_id: Uuid,
    conn: DBPool,
) -> Result<Entry, &'static str> {
    ImageService::create_from_base(raw_entry.photo, &conn)
        .map(|file| ImageService::gen_link(file))
        .map(|link| NewEntry {
            name: raw_entry.name,
            voltage: raw_entry.voltage,
            price: raw_entry.price,
            volume: raw_entry.volume,
            photo: link,
        })
        .map(|new_entry| Entry::create_entry(new_entry, user_id))
        .and_then(|entry| EntryRepo::add_new(entry, &conn))
}

pub fn verify_entry(id: Uuid, state: bool, conn: DBPool) -> Result<Entry, &'static str> {
    EntryRepo::verify(id, Some(state), &conn)
}

pub fn get_verified_entries(conn: DBPool) -> Vec<Entry> {
    EntryRepo::get_verified(&conn)
}

pub fn get_unverified_entries(conn: DBPool) -> Vec<Entry> {
    EntryRepo::get_unverified(&conn)
}

pub fn get_user_entries(user_id: Uuid, conn: DBPool) -> Vec<Entry> {
    EntryRepo::get_users(user_id, &conn)
}
