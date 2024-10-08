use crate::models::BadRequestError;
use crate::schema::user::{self, id};
use crate::users::models::{NewUser, UpdateUser, User};
use crate::users::schema::{CreateUserSchema, ResponseUser, UpdateUserSchema};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Deserialize;
use validator::Validate;

#[utoipa::path(post, path = "/v1/user", tag = "user",
request_body(content = CreateUserSchema, description = "User that should be inserted in the database", content_type = "application/json"),
    responses(
        (status = 201, description = "Create a user", body = ResponseUser)
    )
)]
#[post("/user")]
pub(super) async fn create_user_handler(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    body: web::Json<CreateUserSchema>,
) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let mut connection = db_pool.get().unwrap();

            let hash = pwhash::bcrypt::hash(&body.password).unwrap();

            let new_user = NewUser {
                email: &body.email,
                first_name: &body.first_name,
                last_name: &body.last_name,
                username: &body.username,
                password: hash.as_str(),
            };

            let new_created_user = match diesel::insert_into(user::table)
                .values(&new_user)
                .returning(User::as_returning())
                .get_result(&mut connection)
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

            let response_user = ResponseUser {
                email: new_created_user.email,
                id: new_created_user.id,
                first_name: new_created_user.first_name,
                last_name: new_created_user.last_name,
                username: new_created_user.username,
            };

            HttpResponse::Created().json(response_user)
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
        (status = 200, description = "Update a user", body = ResponseUser)
    ),
    params(
        ("id", description = "User ID")
    )
)]
#[patch("/user/{id}")]
pub(super) async fn update_user_handler(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<UserIdParam>,
    body: web::Json<UpdateUserSchema>,
) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let mut connection = db_pool.get().unwrap();

            let mut update_user = UpdateUser {
                email: body.email.as_deref(),
                first_name: body.first_name.as_deref(),
                last_name: body.last_name.as_deref(),
                username: body.username.as_deref(),
                password: None,
                refresh_token: None,
                refresh_token_expiry: None,
            };

            let hash: String;
            if body.password.is_some() {
                let password_value = body.password.as_deref().unwrap();
                hash = pwhash::bcrypt::hash(password_value).unwrap();
                update_user.password = Some(hash.as_str());
            }

            let updated_user = match diesel::update(user::table)
                .set(update_user)
                .filter(id.eq(path.id))
                .returning(User::as_returning())
                .get_result(&mut connection)
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

            let response_user = ResponseUser {
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
        (status = 200, description = "The user", body = ResponseUser)
    ),
    params(
        ("id", description = "User ID")
    )
)]
#[get("/user/{id}")]
pub(super) async fn get_user_handler(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<UserIdParam>,
) -> impl Responder {
    let mut connection = db_pool.get().unwrap();

    let results = user::table
        .select(User::as_select())
        .filter(id.eq(path.id))
        .first(&mut connection);

    match results {
        Ok(user) => {
            let response_user = ResponseUser {
                email: user.email,
                id: user.id,
                first_name: user.first_name,
                last_name: user.last_name,
                username: user.username,
            };

            HttpResponse::Ok().json(response_user)
        }
        Err(error) => match error {
            diesel::result::Error::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
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
pub(super) async fn delete_user_handler(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<UserIdParam>,
) -> impl Responder {
    let mut connection = db_pool.get().unwrap();

    let _ = diesel::delete(user::table.filter(id.eq(path.id))).execute(&mut connection);

    HttpResponse::Ok()
}

#[utoipa::path(get, path = "/v1/user", tag = "user",
    responses(
        (status = 200, description = "The user", body = Vec<ResponseUser>)
    ),
)]
#[get("/user")]
pub(super) async fn get_users_handler(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let mut connection = db_pool.get().unwrap();

    let results = user::table
        .select(User::as_select())
        .limit(100)
        .load(&mut connection);

    match results {
        Ok(db_users) => {
            let users: Vec<ResponseUser> = db_users
                .iter()
                .map(|user: &User| ResponseUser {
                    email: user.email.clone(),
                    id: user.id,
                    first_name: user.first_name.clone(),
                    last_name: user.last_name.clone(),
                    username: user.username.clone(),
                })
                .collect();

            HttpResponse::Ok().json(users)
        }
        Err(error) => {
            println!("{}", error);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
