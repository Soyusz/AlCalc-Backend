use crate::schema::follows;
use uuid::Uuid;
use diesel::{self,Queryable};
use serde::{Deserialize,Serialize};

#[derive(Clone, Deserialize, Serialize, Queryable, Insertable, Associations)]
#[table_name="follows"]
pub struct Follow {
    pub id: Uuid,
    pub follower_id: Uuid,
    pub followed_id: Uuid
}

impl Follow {
    pub fn create_follow(follower_id: Uuid, followed_id: Uuid) -> Follow {
        Follow {
            id: Uuid::new_v4(),
            follower_id,
            followed_id
        }
    }
}
