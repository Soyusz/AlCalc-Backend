use crate::model::post::{create_post, Post};
use crate::schema::posts;
use crate::schema::posts::dsl::posts as all_posts;
use crate::schema::posts::id as post_id;
use diesel::prelude::*;
use diesel::{self, PgConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Clone, Deserialize, Serialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub location: Option<String>,
    pub title: String,
    pub photos: Vec<String>,
}

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Post> {
    let post_vec = all_posts
        .find(id)
        .load::<Post>(conn)
        .unwrap_or_else(|_| -> Vec<Post> { vec![] });
    if post_vec.len() == 0 {
        None
    } else {
        Some(post_vec[0].clone())
    }
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

pub fn add_new(user_id: Uuid, post: NewPost, conn: &PgConnection) -> Option<Post> {
    let insert_post = create_post(user_id, post);

    let query_res: Result<Vec<Uuid>, _> = diesel::insert_into(posts::table)
        .values(&insert_post)
        .returning(post_id)
        .get_results(conn);

    match query_res {
        Ok(v) => {
            let new_post = get_by_id(v[0], conn).clone();
            match new_post {
                Some(s) => Some(s),
                None => None,
            }
        }
        Err(_) => None,
    }
}
