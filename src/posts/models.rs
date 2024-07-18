use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::post)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub name: Option<String>,
    pub content: String,
    pub image_url: Option<String>,
    pub video_url: Option<String>,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = crate::schema::post)]
pub struct NewPost<'a> {
    pub name: Option<&'a str>,
    pub content: &'a str,
    pub image_url: Option<&'a str>,
    pub video_url: Option<&'a str>,
    pub user_id: i32,
    pub created_at: SystemTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::post)]
pub struct UpdatePost<'a> {
    pub name: Option<&'a str>,
    pub content: Option<&'a str>,
    pub image_url: Option<&'a str>,
    pub video_url: Option<&'a str>,
}
