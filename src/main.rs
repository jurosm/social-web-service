use actix_web::middleware::ErrorHandlers;
use actix_web::{web, App, HttpServer};
use social_web_service::{add_error_header, models::*};
use social_web_service::{config, health, users};
use utoipa::OpenApi;
use utoipa_swagger_ui::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(health::health_handler,users::handler::create_user_handler,users::handler::update_user_handler,users::handler::get_user_handler,users::handler::delete_user_handler,users::handler::get_users_handler),
        components(
            schemas(health::GenericResponse, User, CreateUserSchema, UpdateUserSchema)
        ),
        tags(
            (name = "health", description = "Health check endpoints."),
            (name = "user", description = "User endpoints")
        ),
    )]
    struct ApiDoc;

    HttpServer::new(move || {
        App::new()
            .wrap(ErrorHandlers::new().handler(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                add_error_header,
            ))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(web::scope("/v1").configure(config))
            .service(web::scope("").configure(health::config))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
