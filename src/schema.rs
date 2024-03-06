// @generated automatically by Diesel CLI.

diesel::table! {
    data_tiny (id) {
        id -> Int8,
        base_url -> Varchar,
        short_url -> Varchar,
        created_at -> Timestamp,
        created_by_ip -> Nullable<Varchar>,
    }
}
