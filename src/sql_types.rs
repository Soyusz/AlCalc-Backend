use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DbEnum)]
#[DieselType = "User_role"]
#[PgType = "user_role"]
pub enum UserRoles {
    Admin,
    User,
}
