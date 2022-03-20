use uuid::Uuid;
use diesel::{self, PgConnection};
use crate::model::post::Post;
use crate::model::like::Like;
use crate::model::user::User;
use crate::sql_types;
use crate::schema::likes::{
    self,
    id as like_id,
    dsl::likes as all_likes
};
use crate::schema::users;
use crate::schema::posts;
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

pub fn check_like(
    post_id: Uuid,
    user_id: Uuid,
    conn: &PgConnection
) -> Result<Like,()> {
    all_likes
        .filter(likes::post_id.eq_all(post_id))
        .filter(likes::user_id.eq_all(user_id))
        .first::<Like>(conn)
        .map_err(|_| ())
}


pub fn get_by_post(
    post_id: Uuid,
    conn: &PgConnection
) -> Result<Vec<User>,()> {
    likes::table.inner_join(users::table)
        .filter(likes::post_id.eq_all(post_id))
        .select((users::id,users::email,users::role,users::email_verified,users::name))
        .load::<(Uuid,String,sql_types::UserRoles,bool,String)>(conn)
        .map(|vect| {
                vect.into_iter()
                .map(|el| {
                    User {
                        id: el.0,
                        email: el.1,
                        role: el.2,
                        email_verified: el.3,
                        name: el.4
                    }
                })
                .collect()
        }) 
        .map_err(|_| ())
}

pub fn get_by_user(
    user_id: Uuid,
    conn: &PgConnection
) -> Result<Vec<Post>,()> {
    likes::table.inner_join(posts::table)
        .filter(likes::user_id.eq_all(user_id))
        .select((posts::id,posts::user_id,posts::location,posts::title,posts::photos))
        .load::<(Uuid,Uuid,Option<String>,String,Vec<String>)>(conn)
        .map(|vect| {
                vect.into_iter()
                .map(|el| {
                    Post {
                        id: el.0,
                        user_id: el.1,
                        title: el.3,
                        location: el.2,
                        photos: el.4
                    }
                })
                .collect()
        }) 
        .map_err(|_| ()) 
}