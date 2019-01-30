table! {
    messages (id) {
        id -> Int4,
        content -> Text,
        user_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

table! {
    rooms (id) {
        id -> Int4,
        topic -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    themes (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    topics (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        theme -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        phone_code -> Varchar,
        phone -> Varchar,
        username -> Nullable<Varchar>,
        room_id -> Nullable<Int4>,
        gender -> Nullable<Varchar>,
        validation_code -> Int4,
        created_at -> Timestamp,
    }
}

joinable!(messages -> users (user_id));
joinable!(topics -> themes (theme));
joinable!(users -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    messages,
    rooms,
    themes,
    topics,
    users,
);
