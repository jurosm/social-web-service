use actix_web::{
    get,
    web::{self},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct GenericResponse {
    #[schema(example = "success")]
    pub status: String,
    #[schema(example = "something")]
    pub message: String,
}

#[utoipa::path(get, path = "/health",
    responses(
        (status = 200, description = "Health check", body = GenericResponse)
    )
)]
#[get("/health")]
pub(super) async fn health_handler() -> impl Responder {
    let json_response = &GenericResponse {
        status: "success".to_string(),
        message: "something".to_string(),
    };
    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(health_handler);
}
