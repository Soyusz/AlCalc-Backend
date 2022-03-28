use crate::api::DBPool;
use uuid::Uuid;
use crate::model::post::Post;
use crate::model::user::User;
use crate::model::like::Like;
use crate::db::like as dbPost;

pub fn get_by_post(
    post_id: Uuid,
    conn: DBPool
) -> Result<Vec<User>,&'static str> {
    dbPost::get_by_post(post_id, &conn)
}

pub fn get_by_user(
    user_id: Uuid,
    conn: DBPool
) -> Result<Vec<Post>,&'static str> {
    dbPost::get_by_user(user_id, &conn)
}

pub fn unlike(
    user_id: Uuid,
    post_id: Uuid,
    conn: DBPool
) -> Result<(), &'static str> {
   dbPost::unlike_post(user_id, post_id, &conn)
}

pub fn like(
    user_id: Uuid,
    post_id: Uuid,
    conn: DBPool
) -> Result<Like, &'static str> {
    match dbPost::check_like(post_id, user_id, &conn) {
        Err(_) => dbPost::like_post(user_id, post_id, &conn),
        Ok(l) => Ok(l)
    }
}