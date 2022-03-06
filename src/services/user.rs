use crate::api::DBPool;
use crate::db::user as dbUser;
use crate::model::token as TokenModel;
use crate::model::user::User;
use crate::services::email as EmailService;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn insert_user(user: dbUser::NewUser, conn: DBPool) -> Result<User, bool> {
    EmailService::email_verification(&user.email);
    let res = dbUser::add_new(user, &conn);
    match res {
        Some(e) => Ok(e),
        None => Err(true),
    }
}

pub fn get_user(id: Uuid, conn: DBPool) -> Option<User> {
    dbUser::get_by_id(id, &conn)
}

pub fn get_by_email(email: String, conn: DBPool) -> Option<User> {
    dbUser::get_by_email(email, &conn)
}

pub fn get_all(conn: DBPool) -> Vec<User> {
    dbUser::get_all(&conn)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginCred {
    email: String,
}
pub fn login(cred: LoginCred, conn: DBPool) -> Result<String, String> {
    let user = get_by_email(cred.email, conn);
    let res = match user {
        Some(u) => match TokenModel::create(&u.id) {
            Result::Ok(token) => Result::Ok(token),
            Result::Err(_) => Result::Err("".to_string()),
        },
        None => Result::Err("".to_string()),
    };
    res
}
