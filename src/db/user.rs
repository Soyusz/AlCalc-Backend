use crate::model::user::{create_user, User};
use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use crate::schema::users::id as user_id;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Clone, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<User> {
    let user_vec = all_users
        .find(id)
        .load::<User>(conn)
        .unwrap_or_else(|_| -> Vec<User> { vec![] });
    if user_vec.len() == 0 {
        None
    } else {
        Some(user_vec[0].clone())
    }
}

pub fn get_all(conn: &PgConnection) -> Vec<User> {
    all_users
        .load::<User>(conn)
        .unwrap_or_else(|_| -> Vec<User> { vec![] })
}

pub fn add_new(user: NewUser, conn: &PgConnection) -> Option<User> {
    let insert_user = create_user(user);

    let query_res: Result<Vec<Uuid>, _> = diesel::insert_into(users::table)
        .values(&insert_user)
        .returning(user_id)
        .get_results(conn);

    match query_res {
        Ok(v) => {
            let new_user = get_by_id(v[0], conn).clone();
            match new_user {
                Some(s) => Some(s),
                None => None,
            }
        }
        Err(_) => None,
    }
}
