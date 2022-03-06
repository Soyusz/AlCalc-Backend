table! {
    entries (id) {
        id -> Uuid,
        name -> Varchar,
        price -> Float8,
        voltage -> Float8,
        volume -> Float8,
        verified -> Nullable<Bool>,
        photo -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        email_verified -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    entries,
    users,
);
