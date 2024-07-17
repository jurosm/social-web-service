use crate::models::{
    BadRequestError, CreateUserSchema, NewUser, ResponseUser, UpdateUser, UpdateUserSchema, User,
};
use crate::schema::user::{self, id};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Deserialize;
use validator::Validate;

#[utoipa::path(post, path = "/v1/post", tag = "post",
request_body(content = CreateUserSchema, description = "Post that should be created for the authorized user", content_type = "application/json"),
    responses(
        (status = 201, description = "Create a post", body = User)
    )
)]
#[post("/post")] 
pub(super) async fn create_post_handler() -> impl Responder {

}