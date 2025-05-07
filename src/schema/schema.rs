// @generated automatically by Diesel CLI.

diesel::table! {
    pets (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        tutor_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(pets -> users (tutor_id));

diesel::allow_tables_to_appear_in_same_query!(
    pets,
    users,
);
