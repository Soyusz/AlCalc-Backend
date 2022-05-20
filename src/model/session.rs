use crate::model::user::User;
use crate::schema::sessions;
use diesel::{self, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{SystemTime, Duration};

#[derive(Clone,Debug, Deserialize, Serialize, Queryable, Insertable, Associations)]
#[belongs_to(User)]
#[table_name = "sessions"]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub authorized: bool,
    pub expiration: SystemTime
}

impl Session {
    pub fn create_session(user_id: Uuid) -> Session {
        Session {
            id: Uuid::new_v4(),
            user_id,
            authorized: false,
            expiration: SystemTime::now()
                .checked_add(Duration::from_secs(60*60*24*30))
                .unwrap()
        }
    }
}
