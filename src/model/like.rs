use crate::schema::likes;
use uuid::Uuid;
use diesel::{self,Queryable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Queryable, Insertable, Associations)]
#[table_name="likes"]
pub struct Like {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid 
}

impl Like {
    pub fn create_like(post_id: Uuid, user_id: Uuid) -> Like {
        Like {
            id: Uuid::new_v4(),
            post_id,
            user_id
        }
    }
}
