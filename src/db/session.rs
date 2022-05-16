use crate::model::session::Session;
use crate::schema::sessions::{self, table as all_sessions};
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Session> {
    all_sessions.find(id).first::<Session>(conn).ok()
}

pub fn add_new(session: Session, conn: &PgConnection) -> Result<Session, &'static str> {
    diesel::insert_into(sessions::table)
        .values(&session)
        .get_result::<Session>(conn)
        .map_err(|_| "Insert failed")
}

pub fn authorize(session_id: Uuid, conn: &PgConnection) -> Result<Session,&'static str> {
    diesel::update(sessions::table.find(session_id))
        .set(sessions::authorized.eq(true))
        .get_result::<Session>(conn)
        .map_err(|_| "Cannot authorize session")
}
