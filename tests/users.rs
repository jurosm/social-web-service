use std::ops::Not;

use actix_web::{
    test,
    web::{self},
    App,
};
use fake::{faker::internet::raw::SafeEmail, locales::EN, Fake};
use social_web_service::{
    get_connection_pool,
    models::{NewUser, ResponseUser},
};

#[actix_web::test]
async fn test_users_crud() {
    let pool = get_connection_pool();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/v1").configure(social_web_service::config)),
    )
    .await;

    let req = test::TestRequest::with_uri("/v1/user").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: Vec<ResponseUser> = test::read_body_json(resp).await;

    assert!(!body.is_empty(), "get users not empty");

    // Find a better way to do this
    let fake_email: String = SafeEmail(EN).fake();

    let new_user: NewUser = NewUser {
        email: &fake_email,
        first_name: "Misko",
        last_name: "Miskovic",
        username: "miskopisko",
        password: "1234",
    };

    let req = test::TestRequest::post()
        .uri("/v1/user")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "create single user");

    let response_body = test::read_body(resp).await;

    let created_user: ResponseUser = serde_json::from_slice(&response_body).unwrap();

    assert!(created_user.email.eq(&new_user.email), "email is correct");
    assert!(
        created_user.first_name.unwrap().eq(&new_user.first_name),
        "first name is correct"
    );
    assert!(
        created_user.last_name.unwrap().eq(&new_user.last_name),
        "last name is correct"
    );
    assert!(
        created_user.username.unwrap().eq(&new_user.username),
        "username is correct"
    );

    let req = test::TestRequest::patch()
        .uri(format!("/v1/user/{}", created_user.id.to_string()).as_str())
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "update single user");

    let req = test::TestRequest::get()
        .uri(format!("/v1/user/{}", created_user.id.to_string()).as_str())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "fetch single user");

    let req = test::TestRequest::post()
        .uri("/v1/user")
        .set_json(&new_user)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success().not(),
        "cannot create a user with the an existent email"
    );

    let req = test::TestRequest::delete()
        .uri(format!("/v1/user/{}", created_user.id.to_string()).as_str())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "user deleted");
}
