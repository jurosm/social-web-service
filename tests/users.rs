use actix_web::{test, web, App};
use social_web_service::models::User;

#[actix_web::test]
async fn test_users_crud() {
    let app = test::init_service(
        App::new().service(web::scope("/v1").configure(social_web_service::config)),
    )
    .await;

    let req = test::TestRequest::with_uri("/v1/user").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: Vec<User> = test::read_body_json(resp).await;

    assert!(!body.is_empty(), "get users not empty");
}
