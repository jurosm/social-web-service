use std::time::SystemTime;

use crate::models::{BadRequestError, Claims};
use crate::posts::models::{NewPost, Post, UpdatePost};
use crate::posts::schema::{CreatePostSchema, ResponsePost};
use crate::schema::post::{self, id, user_id};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Deserialize;
use validator::Validate;

use super::schema::UpdatePostSchema;

#[utoipa::path(post, path = "/v1/post", tag = "post",
request_body(content = CreatePostSchema, description = "Post that should be created for the authorized user", content_type = "application/json"),
    responses(
        (status = 201, description = "Create a post", body = ResponsePost)
    )
)]
#[post("/post")]
pub(super) async fn create_post_handler(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    body: web::Json<CreatePostSchema>,
    user: Claims,
) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let mut connection = db_pool.get().unwrap();

            let new_post: NewPost = NewPost {
                name: body.name.as_deref(),
                content: &body.content,
                image_url: body.image_url.as_deref(),
                video_url: body.video_url.as_deref(),
                user_id: user.id,
                created_at: SystemTime::now(),
            };

            let new_created_post: Post = match diesel::insert_into(post::table)
                .values(&new_post)
                .returning(Post::as_returning())
                .get_result(&mut connection)
            {
                Ok(entity) => entity,
                Err(error) => {
                    println!("{}", error);

                    return HttpResponse::BadRequest()
                        .json(BadRequestError {
                            message: "Issue with creating the post.",
                            error: "post.create.some_issue",
                        })
                        .into();
                }
            };

            let response_post: ResponsePost = ResponsePost {
                id: new_created_post.id,
                content: Some(new_created_post.content),
                image_url: new_created_post.image_url,
                video_url: new_created_post.video_url,
                name: new_created_post.name,
            };

            HttpResponse::Ok().json(response_post)
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[derive(Deserialize)]
struct PostIdParam {
    id: i32,
}

#[utoipa::path(patch, path = "/v1/post/{id}", tag = "post",
request_body(content = UpdatePostSchema, description = "Post that should be updated for the authorized user", content_type = "application/json"),
    responses(
        (status = 200, description = "Update a post", body = ResponsePost)
    )
)]
#[patch("/post/{id}")]
pub(super) async fn update_post_handler(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<PostIdParam>,
    body: web::Json<UpdatePostSchema>,
    user: Claims,
) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let mut connection = db_pool.get().unwrap();

            let updated_post = UpdatePost {
                name: body.name.as_deref(),
                content: body.content.as_deref(),
                image_url: body.image_url.as_deref(),
                video_url: body.video_url.as_deref(),
            };

            let post = match diesel::update(post::table)
                .set(updated_post)
                .filter(id.eq(path.id).and(user_id.eq(user.id)))
                .returning(Post::as_returning())
                .get_result(&mut connection)
            {
                Ok(entity) => entity,
                Err(_e) => {
                    return HttpResponse::BadRequest()
                        .json(BadRequestError {
                            message: "User does not exists..",
                            error: "post.create.user_does_not_exists",
                        })
                        .into();
                }
            };

            let response_post = ResponsePost {
                id: post.id,
                content: Some(post.content),
                image_url: post.image_url,
                name: post.name,
                video_url: post.video_url,
            };

            HttpResponse::Ok().json(response_post)
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(get, path = "/v1/post/{id}", tag = "post",
    responses(
        (status = 200, description = "Get a post", body = ResponsePost)
    ),
    params(
        ("id", description = "Post ID")
    )
)]
#[get("/post/{id}")]
pub(super) async fn get_post_handler(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<PostIdParam>,
    _user: Claims,
) -> impl Responder {
    let mut connection = db_pool.get().unwrap();

    let results = post::table
        .select(Post::as_select())
        .filter(id.eq(path.id))
        .first(&mut connection)
        .expect("Error fetching a post");

    let response_post = ResponsePost {
        id: results.id,
        content: Some(results.content),
        image_url: results.image_url,
        name: results.name,
        video_url: results.video_url,
    };

    HttpResponse::Ok().json(response_post)
}

#[utoipa::path(delete, path = "/v1/post/{id}", tag = "post",
    responses(
        (status = 200, description = "Delete a post")
    ),
    params(
        ("id", description = "Post ID")
    )
)]
#[delete("/post/{id}")]
pub(super) async fn delete_post_handler(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<PostIdParam>,
    user: Claims,
) -> impl Responder {
    let mut connection = db_pool.get().unwrap();

    let _ = diesel::delete(post::table.filter(id.eq(path.id).and(user_id.eq(user.id))))
        .execute(&mut connection);

    HttpResponse::Ok()
}

#[utoipa::path(get, path = "/v1/post", tag = "post",
    responses(
        (status = 200, description = "Get list of posts", body = Vec<ResponsePost>)
    )
)]
#[get("/post")]
pub(super) async fn get_posts_handler(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut connection = db_pool.get().unwrap();

    let results = post::table
        .select(Post::as_select())
        .limit(100)
        .load(&mut connection)
        .expect("Failed fetching posts");

    let posts: Vec<ResponsePost> = results
        .iter()
        .map(|post: &Post| ResponsePost {
            id: post.id,
            content: Some(post.content.to_owned()),
            image_url: post.image_url.to_owned(),
            name: post.name.to_owned(),
            video_url: post.video_url.to_owned(),
        })
        .collect();

    HttpResponse::Ok().json(posts)
}
