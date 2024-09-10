use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::{ObjectBuilder, Schema},
    PartialSchema, ToSchema,
};
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

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct ResponsePost {
    pub id: i32,
    pub name: Option<String>,
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub video_url: Option<String>,
}

impl PartialSchema for ResponsePost {
    fn schema() -> utoipa::openapi::RefOr<Schema> {
        ObjectBuilder::new().into()
    }
}
