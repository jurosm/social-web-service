use diesel::r2d2::{ConnectionManager, Pool};
use std::env;
use std::time::Duration;
use uuid::Uuid;

use super::schema::*;

use crate::models::{BadRequestError, Claims};
use crate::schema::user::{self, email, refresh_token, refresh_token_expiry};
use crate::users::models::User;
use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

#[utoipa::path(post, path = "/v1/auth/login", tag = "auth",
request_body(content = UserLoginSchema, description = "Login with user credentials", content_type = "application/json"),
    responses(
        (status = 200, description = "Login with an user", body = UserLoginResponseSchema)
    )
)]
#[post("/auth/login")]
pub async fn login(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    body: web::Json<UserLoginSchema>,
) -> impl Responder {
    let mut connection = db_pool.get().unwrap();

    let user = user::table
        .select(User::as_select())
        .filter(email.eq(&body.email))
        .first(&mut connection)
        .expect("Error fetching a user");

    let is_correct_password = pwhash::bcrypt::verify(&body.password, &user.password);

    if is_correct_password {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let claims = Claims {
            id: user.id,
            exp: since_the_epoch.as_secs() as usize + 7200,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(env::var("JWT_SIGN_PRIVATE_KEY").unwrap().as_bytes()),
        )
        .unwrap();

        let r_token = Uuid::new_v4().to_string();
        let token_expiry = SystemTime::now() + Duration::from_millis(60000 * 120);

        let _ = diesel::update(user::table)
            .set((
                refresh_token.eq(&r_token),
                refresh_token_expiry.eq(&token_expiry),
            ))
            .filter(email.eq(&body.email))
            .execute(&mut connection);

        HttpResponse::Ok().json(UserLoginResponseSchema {
            token,
            refresh_token: r_token,
        })
    } else {
        HttpResponse::BadRequest().json(BadRequestError {
            message: "User does not exists.",
            error: "user.login.not_exists",
        })
    }
}

#[utoipa::path(post, path = "/v1/auth/refresh", tag = "auth",
request_body(content = RefreshTokenSchema, description = "Get new access token with a refresh token", content_type = "application/json"),
    responses(
        (status = 200, description = "Login with an user", body = RefreshTokenResponseSchema)
    )
)]
#[post("/auth/refresh")]
pub async fn refresh(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    body: web::Json<RefreshTokenSchema>,
) -> impl Responder {
    let mut connection = db_pool.get().unwrap();

    let result = user::table
        .select(User::as_select())
        .filter(refresh_token.eq(&body.refresh_token))
        .first(&mut connection)
        .optional()
        .expect("Error fetching a user");

    if result.is_none() {
        HttpResponse::BadRequest().json(BadRequestError {
            message: "User does not exists.",
            error: "user.login.not_exists",
        })
    } else {
        let user_data: User = result.unwrap();

        let expiry_date = user_data.refresh_token_expiry.unwrap();

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        if since_the_epoch.as_secs()
            > expiry_date
                .duration_since(UNIX_EPOCH)
                .expect("System time went backwards")
                .as_secs()
        {
            return HttpResponse::BadRequest().json(BadRequestError {
                message: "Refresh token expired.",
                error: "user.login.refresh_token_expired",
            });
        }

        let claims = Claims {
            id: user_data.id,
            exp: since_the_epoch.as_secs() as usize + 7200,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(env::var("JWT_SIGN_PRIVATE_KEY").unwrap().as_bytes()),
        )
        .unwrap();

        HttpResponse::Ok().json(RefreshTokenResponseSchema { token })
    }
}
