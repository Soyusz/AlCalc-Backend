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
use crate::db::post::get_by_id as get_post_by_id;
use crate::db::user::get_by_id as get_user_by_id;
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
) -> Result<(),()> {
    let res  = diesel::delete(all_likes
        .filter(likes::post_id.eq_all(post_id))
        .filter(likes::user_id.eq_all(user_id)))
        .execute(conn);

    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub fn like_post(
    user_id: Uuid,
    post_id: Uuid,
    conn: &PgConnection
) -> Option<Like> {
    let insert_like = Like::create_like(post_id,user_id);

    let query_res = diesel::insert_into(likes::table)
        .values(&insert_like)
        .returning(like_id)
        .get_results(conn);

    match query_res {
        Ok(v) => {
            let new_like = get_by_id(v[0],conn).clone();
            match new_like {
                Some(s) => Some(s),
                None => None,
            }
        },
        Err(_) => None
    }
}

pub fn get_by_post(
    post_id: Uuid,
    conn: &PgConnection
) -> Vec<User> {
    all_likes
        .filter(likes::post_id.eq_all(post_id))
        .load::<Like>(conn)
        .unwrap_or_else(|_| -> Vec<Like> {vec![]})
        .into_iter() 
        .map(|u| -> User {get_user_by_id(u.id,&conn).unwrap()})
        .collect()
         
}

pub fn get_by_user(
    user_id: Uuid,
    conn: &PgConnection
) -> Vec<Post> {
    all_likes
        .filter(likes::user_id.eq_all(user_id))
        .load::<Like>(conn)
        .unwrap_or_else(|_| -> Vec<Like> {vec![]})
        .into_iter() 
        .map(|u| -> Post {get_post_by_id(u.id,&conn).unwrap()})
        .collect()
}