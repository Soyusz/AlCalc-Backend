use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use rocket::request::FromFormValue;
use rocket::http::RawStr;
use std::str::FromStr;
use std::num::ParseIntError;

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
/*
impl FromStr for EntryLabel {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Beer"
        }
        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

impl<'v> FromFormValue<'v> for EntryLabel{
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<EntryLabel, &'v RawStr> {
        match form_value.parse::<EntryLabel>() {
        }
    }
}

*/