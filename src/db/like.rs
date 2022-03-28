use uuid::Uuid;
use diesel::{self, PgConnection};
use crate::model::post::Post;
use crate::model::like::Like;
use crate::model::user::User;
use crate::schema::likes::{
    self,
    id as like_id,
    dsl::likes as all_likes
};
use crate::schema::users;
use crate::schema::posts;
use diesel::prelude::*;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Like> {
    let vec = all_likes
        .find(id)
        .load::<Like>(conn)
        .unwrap_or_else(|_| -> Vec<Like> { vec![] });
    if vec.len() == 0 {
        None
    } else {
        Some(vec[0].clone())
    }
}

pub fn unlike_post(
    user_id: Uuid,
    post_id: Uuid,
    conn: &PgConnection
) -> Result<(), &'static str> {
    diesel::delete(all_likes
        .filter(likes::post_id.eq_all(post_id))
        .filter(likes::user_id.eq_all(user_id)))
        .execute(conn)
        .map(|_| ())
        .map_err(|_| "Query failed")
}

pub fn like_post(
    user_id: Uuid,
    post_id: Uuid,
    conn: &PgConnection
) -> Result<Like, &'static str> {
    let insert_like = Like::create_like(post_id,user_id);
    diesel::insert_into(likes::table)
        .values(&insert_like)
        .returning(like_id)
        .get_results(conn)
        .map_err(|_| ())
        .and_then(|v|  get_by_id(v[0],conn).ok_or(()) )
        .map_err(|_| "Query failed")
}

pub fn check_like(
    post_id: Uuid,
    user_id: Uuid,
    conn: &PgConnection
) -> Result<Like,()> {
    all_likes
        .filter(likes::post_id.eq_all(post_id))
        .filter(likes::user_id.eq_all(user_id))
        .first::<Like>(conn)
        .map_err(|_| ())
}


pub fn get_by_post(
    post_id: Uuid,
    conn: &PgConnection
) -> Result<Vec<User>,&'static str> {
    likes::table.inner_join(users::table)
        .filter(likes::post_id.eq_all(post_id))
        .select((users::id, users::name, users::email, users::email_verified, users::role, users::photo))
        .load::<User>(conn)
        .map_err(|_| "Query failed")
}

pub fn get_by_user(
    user_id: Uuid,
    conn: &PgConnection
) -> Result<Vec<Post>, &'static str> {
    likes::table.inner_join(posts::table)
        .filter(likes::user_id.eq_all(user_id))
        .select((posts::id,posts::user_id,posts::location,posts::title,posts::photos))
        .load::<Post>(conn)
        .map_err(|_| "Query failed") 
}