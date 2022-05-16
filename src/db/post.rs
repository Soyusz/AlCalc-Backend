use crate::model::post::Post;
use crate::schema::posts::{self, table as all_posts};
use diesel::prelude::*;
use diesel::{self, PgConnection};
use uuid::Uuid;

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Option<Post> {
    all_posts.find(id).first::<Post>(conn).ok()
}

pub fn get_all(conn: &PgConnection) -> Vec<Post> {
    all_posts
        .load::<Post>(conn)
        .unwrap_or(vec![])
}

pub fn get_by_user(user_id: Uuid, conn: &PgConnection) -> Vec<Post> {
    all_posts
        .filter(posts::user_id.eq(user_id))
        .load::<Post>(conn)
        .unwrap_or(vec![])
}

pub fn add_new(post: Post, conn: &PgConnection) -> Result<Post, &'static str> {
    diesel::insert_into(posts::table)
        .values(&post)
        .get_result::<Post>(conn)
        .map_err(|_| "Insert failed")
}
