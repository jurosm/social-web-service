// @generated automatically by Diesel CLI.

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
        #[max_length = 255]
        refresh_token_expiry -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Varchar,
    }
}
