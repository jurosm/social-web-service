use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::web;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub mod auth;
pub mod health;
pub mod models;
pub mod schema;
pub mod users;

pub fn config(conf: &mut web::ServiceConfig) {
    crate::users::controller::config(conf);
    crate::auth::controller::config(conf);
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
        .expect("Could not build connection pool")
}
