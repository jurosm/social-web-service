use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{web, FromRequest, HttpRequest};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation};
use models::Claims;
use std::env;
use std::future::Future;
use std::pin::Pin;

pub mod auth;
pub mod common;
pub mod health;
pub mod models;
pub mod open_api_docs;
pub mod posts;
pub mod schema;
pub mod server;
pub mod users;

pub fn config(conf: &mut web::ServiceConfig) {
    crate::users::controller::config(conf);
    crate::auth::controller::config(conf);
    crate::posts::controller::config(conf);
}

pub fn add_error_header<B>(
    mut res: ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    // body is unchanged, map to "left" slot
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build database connection pool")
}

fn get_claim(token: &String) -> Result<Claims, jsonwebtoken::errors::Error> {
    // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(env::var("JWT_SIGN_PRIVATE_KEY").unwrap().as_bytes()),
        &Validation::default(),
    );

    match result {
        Ok(value) => Ok(value.claims),
        Err(error) => Err(error),
    }
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            Some(hdr) => match hdr.to_str() {
                Ok(h) => h.to_owned(),
                Err(_) => {
                    return Box::pin(async {
                        Err(Self::Error::from(actix_web::error::ErrorUnauthorized(
                            "Invalid header",
                        )))
                    });
                }
            },
            None => {
                return Box::pin(async {
                    Err(Self::Error::from(actix_web::error::ErrorUnauthorized(
                        "Missing header",
                    )))
                });
            }
        };

        Box::pin(async move {
            match auth_header.split(" ").nth(1) {
                Some(token) => match get_claim(&token.to_string()) {
                    Ok(claims) => Ok(claims),
                    Err(_) => Err(Self::Error::from(actix_web::error::ErrorUnauthorized(
                        "Invalid token",
                    ))),
                },
                None => Err(Self::Error::from(actix_web::error::ErrorUnauthorized(
                    "Missing Bearer token",
                ))),
            }
        })
    }
}
