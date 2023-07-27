// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    conversations (id) {
        id -> Uuid,
        #[max_length = 255]
        role -> Varchar,
        chat_id -> Uuid,
        message -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(conversations -> chats (chat_id));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    conversations,
);
