// @generated automatically by Diesel CLI.

pub mod store {
    diesel::table! {
        store.file (id) {
            id -> Int4,
            name -> Varchar,
            identifier -> Varchar,
            size -> Int8,
            mime_type -> Varchar,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }

    diesel::table! {
        store.file_tag (file_id, tag_id) {
            file_id -> Int4,
            tag_id -> Int4,
        }
    }

    diesel::table! {
        store.tag (id) {
            id -> Int4,
            name -> Varchar,
            created_at -> Timestamp,
            updated_at -> Timestamp,
        }
    }

    diesel::joinable!(file_tag -> file (file_id));
    diesel::joinable!(file_tag -> tag (tag_id));

    diesel::allow_tables_to_appear_in_same_query!(
        file,
        file_tag,
        tag,
    );
}
