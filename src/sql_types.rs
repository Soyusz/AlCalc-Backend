use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DbEnum, PartialEq)]
#[DieselType = "User_role"]
#[PgType = "user_role"]
pub enum UserRoles {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize, DbEnum, PartialEq)]
#[DieselType = "Entry_label"]
#[PgType = "entry_label"]
pub enum EntryLabel {
    Beer,
    Vodka,
    Whiskey,
    Wine,
    Champagne,
    Gin,
    Cider,
    Rum,
    Tequila,
    Absinthe,
    Brandy,
    Liqueur,
    Sake,
    Bourbon,
    ScotchWhisky,
    IrishWhiskey,
    Other,
}
