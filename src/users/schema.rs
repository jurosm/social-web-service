use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Validate)]
pub struct CreateUserSchema {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Validate)]
pub struct UpdateUserSchema {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct ResponseUser {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: String,
}
