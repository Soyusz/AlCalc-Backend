use crate::api::DBPool;
use crate::db::post as PostRepo;
use crate::model::post::{NewPost, Post};
use crate::services::image as ImageService;
use crate::db::follow as FollowRepo;
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

pub fn get_feed(_user_id: Uuid, conn: &DBPool) -> Vec<Post> {
    PostRepo::get_all(&conn)
}

pub fn get_feed_followed(user_id: Uuid, conn: &DBPool) -> Vec<Post> {
    let followed_id = FollowRepo::get_user_followed(user_id, conn)
        .into_iter()
        .map(|user| user.id)
        .collect();
    PostRepo::get_all_followed(followed_id, conn)
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
            photos,
        })
        .map(|new_post| Post::create_post(user_id, new_post))
        .and_then(|post| PostRepo::add_new(post, &conn))
}
