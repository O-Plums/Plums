table! {
    rooms (id) {
        id -> Int4,
        subject -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        phone -> Text,
        validation_code -> Int4,
        room_id -> Nullable<Int4>,
    }
}

joinable!(users -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    rooms,
    users,
);
