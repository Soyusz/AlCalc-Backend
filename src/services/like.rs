use crate::api::DBPool;
use crate::db::like as LikeRepo;
use crate::model::like::Like;
use crate::model::post::Post;
use crate::model::user::User;
use uuid::Uuid;

pub fn get_by_post(post_id: Uuid, conn: DBPool) -> Result<Vec<User>, &'static str> {
    Ok(LikeRepo::get_users_by_post(post_id, &conn))
}

pub fn get_by_user(user_id: Uuid, conn: DBPool) -> Result<Vec<Post>, &'static str> {
    Ok(LikeRepo::get_posts_by_user(user_id, &conn))
}

pub fn unlike(user_id: Uuid, post_id: Uuid, conn: DBPool) -> Result<Like, &'static str> {
    LikeRepo::get_like(post_id, user_id, &conn)
        .ok_or("Cannot find like")
        .and_then(|like| LikeRepo::unlike_post(like, &conn))
}

pub fn like(user_id: Uuid, post_id: Uuid, conn: DBPool) -> Result<Like, &'static str> {
    match LikeRepo::get_like(post_id, user_id, &conn) {
        Some(like) => Ok(like),
        None => LikeRepo::like_post(Like::create_like(post_id, user_id), &conn),
    }
}
