use crate::api::DBPool;
use crate::db::user as dbUser;
use crate::model::user::User;
use crate::services::email as EmailService;
use uuid::Uuid;

pub fn insert_user(user: dbUser::NewUser, conn: DBPool) -> Result<User, bool> {
    let res = dbUser::add_new(user, &conn);
    EmailService::email_verification(user.email);
    match res {
        Some(e) => Ok(e),
        None => Err(true),
    }
}

pub fn get_user(id: Uuid, conn: DBPool) -> Option<User> {
    dbUser::get_by_id(id, &conn)
}

pub fn get_all(conn: DBPool) -> Vec<User> {
    dbUser::get_all(&conn)
}
