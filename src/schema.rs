use diesel::{joinable,table,allow_tables_to_appear_in_same_query};

table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
        user_id -> Int4,
        created_at -> Timestamp,
        catagory_id -> Nullable<Int4>,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Text,
        
    }
}

joinable!(posts -> users (user_id));
joinable!(posts -> categories (catagory_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
    categories,
);