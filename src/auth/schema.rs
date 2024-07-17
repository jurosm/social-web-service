use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Validate)]
pub struct UserLoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct UserLoginResponseSchema {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Validate)]
pub struct RefreshTokenSchema {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct RefreshTokenResponseSchema {
    pub token: String,
}
