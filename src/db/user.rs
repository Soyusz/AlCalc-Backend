use crate::model::user::{create_user, User, NewUser};
use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use crate::schema::users::id as user_id;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<User> {
    all_users
    .find(id)
    .first::<User>(conn)
    .ok()
}

pub fn get_by_email(email: String, conn: &PgConnection) -> Option<User> {
    all_users
        .filter(users::email.eq_all(email))
        .first::<User>(conn)
        .ok()
}

pub fn get_all(conn: &PgConnection) -> Vec<User> {
    all_users
        .load::<User>(conn)
        .unwrap_or_else(|_| -> Vec<User> { vec![] })
}

pub fn add_new(user: NewUser, conn: &PgConnection) -> Result<User, &'static str> {
 let new_user  = create_user(user);
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(user_id)
        .get_results(conn)
        .map_err(|_| () )
        .and_then( |v| get_by_id(v[0], conn ).ok_or(()) )
        .map_err(|_| "Insert failed")
}

pub fn update_photo(id: Uuid, new_photo: Option<String>, conn: &PgConnection) -> Result<User, &'static str> {
    use crate::schema::users::photo;
    diesel::update(all_users.find(id))
        .set(photo.eq(new_photo))
        .get_result::<User>(conn)
        .map_err(|_| "Cannot update user")
}