use crate::api::DBPool;
use crate::db::user as dbUser;
use crate::model::token as TokenModel;
use crate::model::user::{NewUser, User};
use crate::services::email as EmailService;
use crate::services::image as ImageService;
use crate::sql_types::UserRoles;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn prepare_new_user(user: NewUser) -> Result<NewUser, &'static str> {
    Ok(NewUser {
        email: user.email.to_lowercase(),
        ..user
    })
}

pub fn insert_user(raw_user: NewUser, conn: DBPool) -> Result<User, &'static str> {
    prepare_new_user(raw_user).and_then(|new_user| {
        dbUser::add_new(new_user, &conn).map(|user| {
            EmailService::email_verification(&user.email);
            user
        })
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
    let email = cred.email.to_lowercase();
    println!("email: {}", email);

    get_by_email(email, conn)
        .ok_or("user not found")
        .and_then(|u| TokenModel::create(&u.id))
}

pub fn update_photo(photo: String, user_id: Uuid, conn: &DBPool) -> Result<User, &'static str> {
    ImageService::create_from_base(photo, conn)
        .map(|i| ImageService::gen_link(i))
        .and_then(|link| dbUser::update_photo(user_id, Some(link), conn))
}
