use crate::api::utils::structs::LoginCred;
use crate::api::DBPool;
use crate::db::user as UserRepo;
use crate::model::user::{NewUser, User};
use crate::services::email as EmailService;
use crate::services::image as ImageService;
use crate::services::jwt_token as JwtTokenService;
use crate::services::token as TokenService;
use crate::sql_types::UserRoles;
use crate::types::token::VerifyAccountPayload;
use uuid::Uuid;

pub fn prepare_new_user(user: NewUser) -> Result<NewUser, &'static str> {
    Ok(NewUser {
        email: user.email.to_lowercase(),
        ..user
    })
}

pub fn insert_user(raw_user: NewUser, conn: DBPool) -> Result<User, &'static str> {
    prepare_new_user(raw_user)
        .map(|new_user| User::create_user(new_user))
        .and_then(|user| UserRepo::add_new(user, &conn))
        .and_then(|user| {
            EmailService::email_verification(&user)?;
            Ok(user)
        })
}

pub fn get_user(id: Uuid, conn: DBPool) -> Option<User> {
    UserRepo::get_by_id(id, &conn)
}

pub fn get_by_email(email: String, conn: &DBPool) -> Option<User> {
    UserRepo::get_by_email(email, &conn)
}

pub fn check_admin(user_id: Uuid, conn: &DBPool) -> Result<User, &'static str> {
    UserRepo::get_by_id(user_id, &conn)
        .ok_or("Invalid user_id")
        .and_then(|u| match u.role == UserRoles::Admin {
            true => Ok(u),
            false => Err("Unauthorized"),
        })
}

pub fn get_all(user_id: Uuid, conn: DBPool) -> Result<Vec<User>, &'static str> {
    check_admin(user_id, &conn).map(|_| UserRepo::get_all(&conn))
}

pub fn update_photo(photo: String, user_id: Uuid, conn: &DBPool) -> Result<User, &'static str> {
    ImageService::create_from_base(photo, conn)
        .map(|i| ImageService::gen_link(i))
        .and_then(|link| UserRepo::update_photo(user_id, link, conn))
}

pub fn verify_account(token: String, conn: &DBPool) -> Result<User, &'static str> {
    JwtTokenService::validate::<VerifyAccountPayload>(token)
        .and_then(|payload| UserRepo::get_by_id(payload.user_id, &conn).ok_or("Cannot fetch user"))
        .and_then(|user| UserRepo::verify_email(user.id, &conn))
}
