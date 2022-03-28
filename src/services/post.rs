use crate::api::DBPool;
use crate::db::post::{self as PostRepo, NewPost};
use crate::model::post::Post;
use uuid::Uuid;
use crate::services::image as ImageService;

pub fn get_all(conn: DBPool) -> Vec<Post> {
    PostRepo::get_all(&conn)
}

pub fn get_by_user(user_id: Uuid, conn: &DBPool) -> Vec<Post> {
    PostRepo::get_by_user(user_id, &conn)
}

pub fn get_by_id(post_id: Uuid, conn: &DBPool) -> Option<Post> {
    PostRepo::get_by_id(post_id, &conn)
}

pub fn get_feed(_: Uuid, conn: &DBPool) -> Vec<Post> {
    PostRepo::get_all(&conn)
}

pub fn insert(post: NewPost, user_id: Uuid, conn: DBPool) -> Result<Post, &'static str> {
    PostRepo::add_new(user_id, post, &conn)
}
