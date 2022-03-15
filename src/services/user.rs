use crate::api::DBPool;
use crate::db::user as dbUser;
use crate::model::token as TokenModel;
use crate::model::user::User;
use crate::services::email as EmailService;
use crate::sql_types::UserRoles;
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

pub fn check_admin(user_id: Uuid, conn: &DBPool) -> bool {
    let user = dbUser::get_by_id(user_id, &conn);
    match user {
        Some(u) => u.role == UserRoles::Admin,
        None => false,
    }
}

pub fn get_all(user_id: Uuid, conn: DBPool) -> Result<Vec<User>, ()> {
    match check_admin(user_id, &conn) {
        true => Ok(dbUser::get_all(&conn)),
        false => Err(()),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginCred {
    email: String,
}

pub fn login(cred: LoginCred, conn: DBPool) -> Result<String, String> {
    let user = get_by_email(cred.email, conn);
    match user {
        Some(u) => match TokenModel::create(&u.id) {
            Ok(token) => Ok(token),
            Err(_) => Err(String::from("")),
        },
        None => Err(String::from("")),
    }
}
