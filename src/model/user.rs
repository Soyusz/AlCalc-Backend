use serde::{Deserialize, Serialize};
use diesel::{self, Queryable};
use uuid::Uuid;
use crate::schema::users;

pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Clone,Deserialize,Serialize,Queryable,Insertable)]
#[table_name="users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String
}

impl User {
    pub fn new(user: NewUser) -> User {
        User{
            id: Uuid::new_v4(),
            name: user.name,
            email: user.email,
            password: user.password
            role: user.role
        }
    }
}