use actix_web::test;
use social_web_service::{health::GenericResponse, server};

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(server::create_app()).await;

    let req = test::TestRequest::get().uri("/health").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "service is healthy");

    let response_body = test::read_body(resp).await;

    let response: GenericResponse = serde_json::from_slice(&response_body).unwrap();

    assert!(response.status.eq("success"), "status is correct");
}
