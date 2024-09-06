// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int8,
        #[max_length = 80]
        title -> Varchar,
        completion_status -> Bool,
        date_created -> Timestamp,
        date_modified -> Timestamp,
        description -> Nullable<Text>,
        date_completed -> Nullable<Timestamp>,
        date_deadline -> Nullable<Timestamp>,
    }
}
