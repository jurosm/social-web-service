use crate::posts::schema::ResponsePost;
use utoipa::OpenApi;
use utoipauto::utoipauto;

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
