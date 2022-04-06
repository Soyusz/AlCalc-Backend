use crate::model::like::Like;
use crate::model::post::Post;
use crate::model::user::User;
use crate::schema::likes::{self, table as all_likes};
use crate::schema::posts::{self, table as all_posts};
use crate::schema::users::{self, table as all_users};
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Like> {
    all_likes.find(id).first::<Like>(conn).ok()
}

pub fn unlike_post(like: Like, conn: &PgConnection) -> Result<Like, &'static str> {
    diesel::delete(
        all_likes
            .filter(likes::post_id.eq_all(like.post_id))
            .filter(likes::user_id.eq_all(like.user_id)),
    )
    .get_result::<Like>(conn)
    .map_err(|_| "Delete failed")
}

pub fn like_post(like: Like, conn: &PgConnection) -> Result<Like, &'static str> {
    diesel::insert_into(likes::table)
        .values(&like)
        .get_result::<Like>(conn)
        .map_err(|_| "Query failed")
}

pub fn get_like(post_id: Uuid, user_id: Uuid, conn: &PgConnection) -> Option<Like> {
    all_likes
        .filter(likes::post_id.eq_all(post_id))
        .filter(likes::user_id.eq_all(user_id))
        .first::<Like>(conn)
        .ok()
}

pub fn get_users_by_post(post_id: Uuid, conn: &PgConnection) -> Vec<User> {
    all_likes
        .inner_join(all_users)
        .filter(likes::post_id.eq_all(post_id))
        .select(users::all_columns)
        .load::<User>(conn)
        .unwrap_or(vec![])
}

pub fn get_posts_by_user(user_id: Uuid, conn: &PgConnection) -> Vec<Post> {
    all_likes
        .inner_join(all_posts)
        .filter(likes::user_id.eq_all(user_id))
        .select(posts::all_columns)
        .load::<Post>(conn)
        .unwrap_or(vec![])
}
