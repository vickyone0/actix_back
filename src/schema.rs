// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        price -> Numeric,
        created_at -> Nullable<Timestamp>,
    }
}
