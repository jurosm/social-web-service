use actix_web::middleware::ErrorHandlers;
use actix_web::{web, App, HttpServer};
use social_web_service::{add_error_header, get_connection_pool};
use social_web_service::{auth, config, health, users, users::schema::*, posts};
use utoipa::OpenApi;
use utoipa_swagger_ui::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(health::health_handler,
            users::handler::create_user_handler,
            users::handler::update_user_handler,
            users::handler::get_user_handler,
            users::handler::delete_user_handler,
            users::handler::get_users_handler,
            posts::handler::get_post_handler,
            posts::handler::get_posts_handler,
            posts::handler::create_post_handler,
            posts::handler::update_post_handler,
            posts::handler::delete_post_handler,
            auth::handler::login,
            auth::handler::refresh),
        components(
            schemas(health::GenericResponse, 
                CreateUserSchema, 
                UpdateUserSchema, 
                ResponseUser, 
                auth::schema::UserLoginSchema, 
                auth::schema::UserLoginResponseSchema,
                auth::schema::RefreshTokenSchema,
                auth::schema::RefreshTokenResponseSchema)
        ),
        tags(
            (name = "health", description = "Health check endpoints."),
            (name = "user", description = "User endpoints"),
            (name = "auth", description = "Auth endpoints"),
            (name = "post", description = "Post endpoints")
        ),
    )]
    struct ApiDoc;

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
