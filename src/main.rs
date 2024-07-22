use actix_web::middleware::ErrorHandlers;
use actix_web::{web, App, HttpServer};
use social_web_service::{add_error_header, get_connection_pool};
use social_web_service::{auth, config, health, posts, users};
use utoipa::OpenApi;
use utoipa_swagger_ui::*;
use utoipauto::utoipauto;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[utoipauto]
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "health", description = "Health check endpoints."),
            (name = "user", description = "User endpoints"),
            (name = "auth", description = "Auth endpoints"),
            (name = "post", description = "Post endpoints")
        ),
    )]
    pub struct ApiDoc;

    let pool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
