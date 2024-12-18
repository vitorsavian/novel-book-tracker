// @generated automatically by Diesel CLI.

diesel::table! {
    novel (id) {
        id -> Uuid,
        #[max_length = 1000]
        name -> Varchar,
        chapter -> Int4,
        finished -> Bool,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}
