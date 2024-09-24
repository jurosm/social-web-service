use actix_web::HttpServer;
use social_web_service::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || server::create_app())
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
