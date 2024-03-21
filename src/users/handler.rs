use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use social_web_service::{
    establish_connection,
    models::{CreateUserSchema, NewUser, UpdateUser, UpdateUserSchema, User},
    schema::user::{self, id},
};
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct BadRequestError<'a> {
    message: &'a str,
    error: &'a str,
}

#[utoipa::path(post, path = "/v1/user", tag = "user",
request_body(content = CreateUserSchema, description = "User that should be inserted in the database", content_type = "application/json"),
    responses(
        (status = 201, description = "Create a user", body = User)
    )
)]
#[post("/user")]
pub(super) async fn create_user_handler(body: web::Json<CreateUserSchema>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let connection = &mut establish_connection();

            let new_user = NewUser {
                email: &body.email,
                first_name: &body.first_name,
                last_name: &body.last_name,
                username: &body.username,
            };

            let new_created_user = match diesel::insert_into(user::table)
                .values(&new_user)
                .returning(User::as_returning())
                .get_result(connection)
            {
                Ok(entity) => entity,
                Err(_e) => {
                    return HttpResponse::BadRequest()
                        .json(BadRequestError {
                            message: "User with that email already exists.",
                            error: "user.create.already_exists",
                        })
                        .into()
                }
            };

            let response_user = User {
                email: new_created_user.email,
                id: new_created_user.id,
                first_name: new_created_user.first_name,
                last_name: new_created_user.last_name,
                username: new_created_user.username,
            };

            HttpResponse::Ok().json(response_user)
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[derive(Deserialize)]
struct UserIdParam {
    id: i32,
}

#[utoipa::path(patch, path = "/v1/user/{id}", tag = "user",
request_body(content = UpdateUserSchema, description = "User that should be updated in the database", content_type = "application/json"),
    responses(
        (status = 200, description = "Update a user", body = User)
    ),
    params(
        ("id", description = "User ID")
    )
)]
#[patch("/user/{id}")]
pub(super) async fn update_user_handler(
    path: web::Path<UserIdParam>,
    body: web::Json<UpdateUserSchema>,
) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let connection = &mut establish_connection();

            let update_user = UpdateUser {
                email: body.email.as_deref(),
                first_name: body.first_name.as_deref(),
                last_name: body.last_name.as_deref(),
                username: body.username.as_deref(),
            };

            let updated_user = match diesel::update(user::table)
                .set(update_user)
                .filter(id.eq(path.id))
                .returning(User::as_returning())
                .get_result(connection)
            {
                Ok(entity) => entity,
                Err(_e) => {
                    return HttpResponse::BadRequest()
                        .json(BadRequestError {
                            message: "User with that email already exists.",
                            error: "user.create.already_exists",
                        })
                        .into();
                }
            };

            let response_user = User {
                email: updated_user.email,
                id: updated_user.id,
                first_name: updated_user.first_name,
                last_name: updated_user.last_name,
                username: updated_user.username,
            };

            HttpResponse::Ok().json(response_user)
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(get, path = "/v1/user/{id}", tag = "user",
    responses(
        (status = 200, description = "The user", body = User)
    ),
    params(
        ("id", description = "User ID")
    )
)]
#[get("/user/{id}")]
pub(super) async fn get_user_handler(path: web::Path<UserIdParam>) -> impl Responder {
    let connection = &mut establish_connection();

    let results = user::table
        .select(User::as_select())
        .filter(id.eq(path.id))
        .first(connection)
        .expect("Error fetching a user");

    let response_user = User {
        email: results.email,
        id: results.id,
        first_name: results.first_name,
        last_name: results.last_name,
        username: results.username,
    };

    HttpResponse::Ok().json(response_user)
}

#[utoipa::path(delete, path = "/v1/user/{id}", tag = "user",
    responses(
        (status = 200, description = "Delete the user")
    ),
    params(
        ("id", description = "User ID")
    )
)]
#[delete("/user/{id}")]
pub(super) async fn delete_user_handler(path: web::Path<UserIdParam>) -> impl Responder {
    let connection = &mut establish_connection();

    diesel::delete(user::table.filter(id.eq(path.id))).execute(connection);

    HttpResponse::Ok()
}

#[utoipa::path(get, path = "/v1/user", tag = "user",
    responses(
        (status = 200, description = "The user", body = Vec<User>)
    ),
)]
#[get("/user")]
pub(super) async fn get_users_handler() -> impl Responder {
    let connection = &mut establish_connection();

    let results = user::table
        .select(User::as_select())
        .limit(100)
        .load(connection)
        .expect("Error fetching the users");

    let users: Vec<User> = results
        .iter()
        .map(|user: &User| User {
            email: user.email.clone(),
            id: user.id,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            username: user.username.clone(),
        })
        .collect();

    HttpResponse::Ok().json(users)
}
