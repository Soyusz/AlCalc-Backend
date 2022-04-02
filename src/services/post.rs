use crate::api::DBPool;
use crate::db::post::{self as PostRepo, NewPost};
use crate::model::post::Post;
use crate::services::image as ImageService;
use uuid::Uuid;

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
    post.photos
        .into_iter()
        .map(|b64| {
            ImageService::create_from_base(b64, &conn).map(|image| ImageService::gen_link(image))
        })
        .collect::<Result<Vec<String>, &'static str>>()
        .map(|photos| NewPost {
            location: post.location,
            title: post.title,
            photos: photos,
        })
        .and_then(|new_post| PostRepo::add_new(user_id, new_post, &conn))
}
