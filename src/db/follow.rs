use crate::model::follow::Follow;
use crate::model::user::User;
use crate::schema::follows::{self, table as all_follows};
use crate::schema::users::{self, table as all_users};
use diesel::prelude::*;
use diesel::{self,PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Follow> {
    all_follows.find(id).first::<Follow>(conn).ok()
}

pub fn follow(
    follow: Follow,
    conn: &PgConnection
) -> Result<Follow,&'static str> {
     diesel::insert_into(follows::table)
         .values(&follow)
         .get_result::<Follow>(conn)
         .map_err(|_| "Insert Failed")
}

pub fn get_follow(
    follower_id: Uuid,
    followed_id: Uuid,
    conn: &PgConnection
) -> Option<Follow> {
    all_follows
        .filter(follows::followed_id.eq_all(followed_id))
        .filter(follows::follower_id.eq_all(follower_id))
        .first::<Follow>(conn)
        .ok()
}

pub fn unfollow(
    follow: Follow,
    conn: &PgConnection
) -> Result<Follow,&'static str> {
    diesel::delete(
        all_follows
            .filter(follows::followed_id.eq_all(follow.followed_id))
            .filter(follows::follower_id.eq_all(follow.follower_id)),
    )
    .get_result::<Follow>(conn)
    .map_err(|_| "Delete failed")
}
pub fn get_user_followers(user_id: Uuid, conn: &PgConnection) -> Vec<User> {
    all_follows
        .inner_join(
            all_users.on(follows::followed_id.eq(users::id))
            )
        .filter(follows::followed_id.eq_all(user_id))
        .select(users::all_columns)
        .load::<User>(conn)
        .unwrap_or(vec![])
}

pub fn get_user_followed(user_id: Uuid, conn: &PgConnection) -> Vec<User> {
    all_follows
        .inner_join(
            all_users.on(follows::follower_id.eq(users::id))
            )
        .filter(follows::follower_id.eq_all(user_id))
        .select(users::all_columns)
        .load::<User>(conn)
        .unwrap_or(vec![])
}
