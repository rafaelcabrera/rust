// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        slug -> Varchar,
        body -> Text,
    }
}
