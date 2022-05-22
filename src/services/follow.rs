use crate::api::DBPool;
use crate::model::follow::Follow;
use crate::db::follow as FollowRepo;
use crate::model::user::User;
use uuid::Uuid;

pub fn get_user_follows(user_id: Uuid, conn: &DBPool) -> Result<Vec<User>,&'static str> {
    Ok(FollowRepo::get_user_followers(user_id, &conn))
}

pub fn get_user_followed(user_id: Uuid, conn: &DBPool) -> Result<Vec<User>,&'static str> {
    Ok(FollowRepo::get_user_followed(user_id, &conn))
}

pub fn unfollow(follower_id: Uuid, followed_id: Uuid, conn: &DBPool) -> Result<Follow ,&'static str> {
    FollowRepo::get_follow(follower_id, followed_id, &conn)
        .ok_or("Cannot find follow")
        .and_then(|follow| FollowRepo::unfollow(follow,&conn))
}

pub fn follow(follower_id: Uuid, followed_id: Uuid, conn: &DBPool) -> Result<Follow, &'static str> {
    match FollowRepo::get_follow(follower_id, followed_id, &conn) {
        Some(follow) => Ok(follow),
        None => FollowRepo::follow(Follow::create_follow(followed_id, follower_id), &conn),
    }
}
