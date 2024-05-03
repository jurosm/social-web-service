use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::web;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod health;
pub mod models;
pub mod schema;
pub mod users;
pub mod auth;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn config(conf: &mut web::ServiceConfig) {
    crate::users::controller::config(conf);
    crate::auth::controller::config(conf);
}

pub fn add_error_header<B>(mut res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    // body is unchanged, map to "left" slot
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
