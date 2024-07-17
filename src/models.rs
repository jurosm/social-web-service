use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: String,
    pub password: String,
    pub refresh_token: Option<String>,
    pub refresh_token_expiry: Option<std::time::SystemTime>,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = crate::schema::user)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::user)]
pub struct UpdateUser<'a> {
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
    pub refresh_token: Option<&'a str>,
    pub refresh_token_expiry: Option<&'a std::time::SystemTime>,
}

#[derive(Serialize, Deserialize)]
pub struct BadRequestError<'a> {
    pub message: &'a str,
    pub error: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
}
