use crate::model::user::User;
use crate::schema::users::{self, table as all_users};
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<User> {
    all_users.find(id).first::<User>(conn).ok()
}

pub fn get_by_email(email: String, conn: &PgConnection) -> Option<User> {
    all_users
        .filter(users::email.eq(email))
        .first::<User>(conn)
        .ok()
}

pub fn get_all(conn: &PgConnection) -> Vec<User> {
    users::table.load::<User>(conn).unwrap_or(vec![])
}

pub fn add_new(user: User, conn: &PgConnection) -> Result<User, &'static str> {
    diesel::insert_into(all_users)
        .values(user)
        .get_result::<User>(conn)
        .map_err(|_| "Insert failed")
}

pub fn delete_user(user_id: Uuid, conn: &PgConnection) -> Result<User, &'static str> {
    diesel::delete(all_users.find(user_id))
        .get_result::<User>(conn)
        .map_err(|_| "Delete failed")
}

pub fn update_photo(
    id: Uuid,
    image_link: String,
    conn: &PgConnection,
) -> Result<User, &'static str> {
    diesel::update(users::table.find(id))
        .set(users::photo.eq(image_link))
        .get_result::<User>(conn)
        .map_err(|_| "Cannot update user")
}
