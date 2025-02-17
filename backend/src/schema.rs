// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Int4,
        content -> Text,
        topic_id -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    topic_tags (topic_id, tag_id) {
        topic_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    topics (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(notes -> topics (topic_id));
diesel::joinable!(topic_tags -> tags (tag_id));
diesel::joinable!(topic_tags -> topics (topic_id));
diesel::joinable!(topics -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    notes,
    tags,
    topic_tags,
    topics,
    users,
);
