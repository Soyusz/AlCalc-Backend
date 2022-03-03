use crate::{db::user::NewUser, schema::users};
use diesel::{self, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
}

pub fn create_user(user: NewUser) -> User {
    User {
        id: Uuid::new_v4(),
        name: user.name,
        email: user.email,
        email_verified: false,
    }
}
