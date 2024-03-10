use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{App, HttpServer};
use social_web_service::models::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::*;

mod health;
mod users;

fn add_error_header<B>(mut res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    // body is unchanged, map to "left" slot
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

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
            // .configure(health::config)
            .configure(users::config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
