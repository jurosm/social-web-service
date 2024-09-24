use actix_web::test;
use fake::{faker::internet::raw::SafeEmail, locales::EN, Fake};
use social_web_service::{auth::schema::*, server, users::models::NewUser};

#[actix_web::test]
async fn tests_user_login() {
    let app = test::init_service(server::create_app()).await;

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

    let credentials = UserLoginSchema {
        email: new_user.email.to_string(),
        password: new_user.password.to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&credentials)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "login with an user");

    let response_body = test::read_body(resp).await;

    let credentials: UserLoginResponseSchema = serde_json::from_slice(&response_body).unwrap();

    assert!(!credentials.token.is_empty(), "token is returned");
    assert!(
        !credentials.refresh_token.is_empty(),
        "refresh token is returned"
    );
}

#[actix_web::test]
async fn tests_user_login_refresh() {
    let app = test::init_service(server::create_app()).await;

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

    let credentials = UserLoginSchema {
        email: new_user.email.to_string(),
        password: new_user.password.to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&credentials)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "login with an user");

    let response_body = test::read_body(resp).await;

    let credentials: UserLoginResponseSchema = serde_json::from_slice(&response_body).unwrap();

    assert!(!credentials.token.is_empty(), "token is returned");
    assert!(
        !credentials.refresh_token.is_empty(),
        "refresh token is returned"
    );

    let refresh_credentials: RefreshTokenSchema = RefreshTokenSchema {
        refresh_token: credentials.refresh_token,
    };

    let req = test::TestRequest::post()
        .uri("/v1/auth/refresh")
        .set_json(&refresh_credentials)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "refresh token with an user");

    let response_body = test::read_body(resp).await;

    let credentials: RefreshTokenResponseSchema = serde_json::from_slice(&response_body).unwrap();

    assert!(!credentials.token.is_empty(), "token is returned");
}
