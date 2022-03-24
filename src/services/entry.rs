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
    let photo = ImageService::create_from_base(entry.photo, &conn);
    match photo {
        Err(s) => Err(s),
        Ok(p) => {
            let new_entry = NewEntry {
                photo: ImageService::gen_link(p),
                name: entry.name,
                voltage: entry.voltage,
                price: entry.price,
                volume: entry.volume 
            };
            add_new(new_entry, user_id, &conn).ok_or("cannot insert entry")
        }
    }
}

pub fn verify_entry(id: Uuid, state: bool, conn: DBPool) -> Result<Entry, &'static str> {
    verify(id, Some(state), &conn).ok_or("cannot verify entry")
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
