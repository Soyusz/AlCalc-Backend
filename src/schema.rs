table! {
    use diesel::sql_types::*;

    entries (id) {
        id -> Uuid,
        name -> Varchar,
        price -> Float8,
        voltage -> Float8,
        volume -> Float8,
        verified -> Nullable<Bool>,
        photo -> Varchar,
        user_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        email_verified -> Bool,
        role -> User_role,
    }
}

joinable!(entries -> users (user_id));

allow_tables_to_appear_in_same_query!(entries, users,);
