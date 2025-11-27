// @generated automatically by Diesel CLI.

diesel::table! {
    complitions (id) {
        id -> Nullable<Integer>,
        habit_id -> Integer,
        done_at -> Timestamp,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    habbits (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        archived_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(complitions -> habbits (habit_id));

diesel::allow_tables_to_appear_in_same_query!(complitions, habbits,);
