use actix_web::test;
use fake::{faker::internet::raw::SafeEmail, locales::EN, Fake};
use social_web_service::{
    auth::schema::{UserLoginResponseSchema, UserLoginSchema},
    common::api::response::ListResponse,
    posts::schema::{CreatePostSchema, ResponsePost, UpdatePostSchema},
    server,
    users::models::NewUser,
};

#[actix_web::test]
async fn tests_posts_crud() {
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

    let new_post: CreatePostSchema = CreatePostSchema {
        content: "Djudjo".to_string(),
        name: Some("Djudjo".to_string()),
        image_url: Some("https://th.bing.com/".to_string()),
        video_url: Some("https://th.bing.com/".to_string()),
    };

    let bearer = format!("Bearer {}", credentials.token);

    let create_post_req = test::TestRequest::post()
        .append_header((actix_web::http::header::AUTHORIZATION, bearer.clone()))
        .uri("/v1/post")
        .set_json(&new_post)
        .to_request();

    let create_post_resp = test::call_service(&app, create_post_req).await;

    assert!(create_post_resp.status().is_success(), "create single post");

    let created_post_string = test::read_body(create_post_resp).await;

    let created_post: ResponsePost = serde_json::from_slice(&created_post_string).unwrap();

    assert!(
        created_post.content.unwrap().eq(&new_post.content),
        "content is correct"
    );

    assert!(
        created_post.name.unwrap().eq(&new_post.name.unwrap()),
        "name is correct"
    );

    assert!(
        created_post
            .image_url
            .unwrap()
            .eq(&new_post.image_url.unwrap()),
        "image_url is correct"
    );

    assert!(
        created_post
            .video_url
            .unwrap()
            .eq(&new_post.video_url.unwrap()),
        "video_url is correct"
    );

    let update_post = UpdatePostSchema {
        content: Some("Test".to_string()),
        image_url: None,
        name: None,
        video_url: None,
    };

    let update_post_req = test::TestRequest::patch()
        .append_header((actix_web::http::header::AUTHORIZATION, bearer.clone()))
        .uri(format!("/v1/post/{}", created_post.id.to_string()).as_str())
        .set_json(update_post)
        .to_request();

    let update_post_resp = test::call_service(&app, update_post_req).await;

    assert!(update_post_resp.status().is_success(), "update post");

    let get_post_req = test::TestRequest::get()
        .uri(format!("/v1/post/{}", created_post.id.to_string()).as_str())
        .to_request();

    let get_post_resp = test::call_service(&app, get_post_req).await;

    assert!(get_post_resp.status().is_success(), "get single post");

    let get_posts_req = test::TestRequest::get().uri("/v1/post").to_request();

    let get_posts_resp = test::call_service(&app, get_posts_req).await;

    assert!(get_posts_resp.status().is_success(), "get post list");

    let get_posts_string = test::read_body(get_posts_resp).await;

    let posts: ListResponse<ResponsePost> = serde_json::from_slice(&get_posts_string).unwrap();

    assert!(posts.data.len() > 0, "post list is not empty");

    let delete_post_req = test::TestRequest::delete()
        .append_header((actix_web::http::header::AUTHORIZATION, bearer.clone()))
        .uri(format!("/v1/post/{}", created_post.id.to_string()).as_str())
        .to_request();

    let delete_post_resp = test::call_service(&app, delete_post_req).await;

    assert!(delete_post_resp.status().is_success(), "delete post");
}
