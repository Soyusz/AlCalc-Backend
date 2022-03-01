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
