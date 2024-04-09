// @generated automatically by Diesel CLI.

diesel::table! {
    locations (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        latitude -> Float8,
        longitude -> Float8,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::joinable!(locations -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(locations, users,);
