// @generated automatically by Diesel CLI.

diesel::table! {
    tweets (id) {
        id -> Int4,
        #[max_length = 140]
        message -> Varchar,
        created_at -> Timestamp,
    }
}
