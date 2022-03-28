use crate::model::post::{create_post, Post};
use crate::schema::posts;
use crate::schema::posts::dsl::posts as all_posts;
use crate::schema::posts::id as post_id;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct NewPost {
    pub location: Option<String>,
    pub title: String,
    pub photos: Vec<String>,
}

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Post> {
    all_posts
        .find(id)
        .first::<Post>(conn)
        .ok()
}

pub fn get_all(conn: &PgConnection) -> Vec<Post> {
    all_posts
        .load::<Post>(conn)
        .unwrap_or_else(|_| -> Vec<Post> { vec![] })
}

pub fn get_by_user(user_id: Uuid, conn: &PgConnection) -> Vec<Post> {
    all_posts
        .filter(posts::user_id.eq(user_id))
        .load::<Post>(conn)
        .unwrap_or_else(|_| -> Vec<Post> { vec![] })
}

pub fn add_new(user_id: Uuid, post: NewPost, conn: &PgConnection) -> Result<Post, &'static str> {
    let new_post = create_post(user_id, post);
    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(post_id)
        .get_results(conn)
        .map_err(|_| ())
        .and_then( |v| get_by_id(v[0], conn ).ok_or(()) )
        .map_err(|_| "Insert failed")
}
