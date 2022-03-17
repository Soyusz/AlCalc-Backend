use crate::{db::post::NewPost, schema::posts};
use diesel::{self, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Queryable, Insertable, Debug)]
#[table_name = "posts"]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub location: Option<String>,
    pub title: String,
    pub photos: Vec<String>,
}

pub fn create_post(user_id: Uuid, post: NewPost) -> Post {
    Post {
        id: Uuid::new_v4(),
        user_id: user_id,
        location: post.location,
        title: post.title,
        photos: post.photos,
    }
}
