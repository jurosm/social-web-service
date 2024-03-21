use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub email: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::user)]
pub struct UpdateUser<'a> {
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Validate)]
pub struct CreateUserSchema {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Validate)]
pub struct UpdateUserSchema {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}
