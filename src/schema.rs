// @generated automatically by Diesel CLI.

diesel::table! {
    post (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        content -> Text,
        created_at -> Timestamp,
        #[max_length = 2048]
        video_url -> Nullable<Varchar>,
        #[max_length = 2048]
        image_url -> Nullable<Varchar>,
        user_id -> Int4,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        refresh_token -> Nullable<Varchar>,
        refresh_token_expiry -> Nullable<Timestamp>,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(post -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(post, user,);
