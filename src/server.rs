use crate::{add_error_header, get_connection_pool, open_api_docs};
use crate::{config, health};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::ErrorHandlers;
use actix_web::{web, App};
use utoipa::OpenApi;
use utoipa_swagger_ui::*;

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let pool = get_connection_pool();

    App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(ErrorHandlers::new().handler(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            add_error_header,
        ))
        .service(
            SwaggerUi::new("/documentation/{_:.*}")
                .url("/api-docs/openapi.json", open_api_docs::ApiDoc::openapi()),
        )
        .service(web::scope("/v1").configure(config))
        .service(web::scope("").configure(health::config))
}
