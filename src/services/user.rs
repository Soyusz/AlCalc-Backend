use crate::api::DBPool;
use crate::db::user as dbUser;
use crate::model::token as TokenModel;
use crate::model::user::{User, NewUser};
use crate::services::email as EmailService;
use crate::sql_types::UserRoles;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn insert_user(user: NewUser, conn: DBPool) -> Result<User, &'static str> {
    dbUser::add_new(user, &conn)
        .ok_or("cannot create user")
        .map(|u| {
            EmailService::email_verification(&u.email);
            u
        })
}

pub fn get_user(id: Uuid, conn: DBPool) -> Option<User> {
    dbUser::get_by_id(id, &conn)
}

pub fn get_by_email(email: String, conn: DBPool) -> Option<User> {
    dbUser::get_by_email(email, &conn)
}

pub fn check_admin(user_id: Uuid, conn: &DBPool) -> Result<User, &'static str> {
    dbUser::get_by_id(user_id, &conn)
        .ok_or("invalid user_id")
        .and_then(|u| match u.role == UserRoles::Admin {
            true => Ok(u),
            false => Err("unauthorized"),
        })
}

pub fn get_all(user_id: Uuid, conn: DBPool) -> Result<Vec<User>, &'static str> {
    check_admin(user_id, &conn).map(|_| dbUser::get_all(&conn))
}
#[derive(Deserialize, Serialize, Debug)]
pub struct LoginCred {
    email: String,
}

pub fn login(cred: LoginCred, conn: DBPool) -> Result<String, &'static str> {
    get_by_email(cred.email, conn)
        .ok_or("user not found")
        .and_then(|u| TokenModel::create(&u.id))
}
