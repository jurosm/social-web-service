use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Validate)]
pub struct CreatePostSchema {
    pub name: Option<String>,
    pub content: String,
    #[validate(url)]
    pub video_url: Option<String>,
    #[validate(url)]
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Validate)]
pub struct UpdatePostSchema {
    pub name: Option<String>,
    pub content: Option<String>,
    #[validate(url)]
    pub video_url: Option<String>,
    #[validate(url)]
    pub image_url: Option<String>,
}
