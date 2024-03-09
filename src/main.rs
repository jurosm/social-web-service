use crate::user::dsl::*;
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use social_web_service::models::*;
use social_web_service::schema::user::{self, username};
use social_web_service::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::*;
use uuid::Uuid;

mod health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(health::health_handler),
        components(
            schemas(health::GenericResponse)
        ),
        tags(
            (name = "health", description = "Health check endpoints.")
        ),
    )]
    struct ApiDoc;

    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .configure(health::config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn db_test_crud() {
    let page_size = 100;

    let connection = &mut establish_connection();
    let results = user::table
        .limit(page_size)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user1 in results {
        println!("{:?}", user1.email);
    }

    let random_id = Uuid::new_v4();

    // Let's now insert a user
    let new_user = NewUser {
        email: &format!("{}@gmail.com", random_id),
        first_name: "oli",
        last_name: "dragoejvic",
        username: "olidrag",
    };

    diesel::insert_into(user::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error adding new user!");

    // Get results, again
    let results = user::table
        .limit(page_size)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user1 in results {
        println!("{:?}", user1.email);
    }

    // Update the user
    let _ = diesel::update(user::table)
        .filter(id.is_not_null())
        .set(username.eq("bohomo"))
        .execute(connection);

    // Get results, again
    let results = user::table
        .limit(page_size)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user1 in results {
        println!("{:?}", user1.username);
    }
}
