use crate::user::dsl::*;
use diesel::prelude::*;
use social_web_service::models::*;
use social_web_service::schema::user::{self, username};
use social_web_service::*;
use uuid::Uuid;

fn main() {
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
