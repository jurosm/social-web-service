use chrono::{DateTime, Utc};
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;
use std::time::Duration;
use uuid::Uuid;

use crate::models::{BadRequestError, User, UserLoginResponseSchema, UserLoginSchema};
use crate::schema::user::{self, email, refresh_token, refresh_token_expiry};
use actix_web::{post, web, HttpResponse, Responder};
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

fn iso_date(time: SystemTime) -> String {
    let now: DateTime<Utc> = time.into();
    now.to_rfc3339()
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: i32,
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
}

#[utoipa::path(post, path = "/v1/auth/login", tag = "auth",
request_body(content = UserLoginSchema, description = "Login with user credentials", content_type = "application/json"),
    responses(
        (status = 200, description = "Login with an user", body = UserLoginResponseSchema)
    )
)]
#[post("/auth/login")]
pub async fn login(
    db_pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    body: web::Json<UserLoginSchema>,
) -> impl Responder {
    let mut connection = db_pool.get().unwrap();

    let user = user::table
        .select(User::as_select())
        .filter(email.eq(&body.email))
        .first(&mut connection)
        .expect("Error fetching a user");

    let is_correct_password = pwhash::bcrypt::verify(&body.password, &user.password);

    if is_correct_password {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let claims = Claims {
            id: user.id,
            exp: since_the_epoch.as_secs() as usize + 7200,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(env::var("JWT_SIGN_PRIVATE_KEY").unwrap().as_bytes()),
        )
        .unwrap();

        let r_token = Uuid::new_v4().to_string();
        let token_expiry = iso_date(SystemTime::now() + Duration::from_millis(60000 * 120));

        let _ = diesel::update(user::table)
            .set((
                refresh_token.eq(&r_token),
                refresh_token_expiry.eq(&token_expiry),
            ))
            .filter(email.eq(&body.email))
            .execute(&mut connection);

        HttpResponse::Ok().json(UserLoginResponseSchema {
            token,
            refresh_token: r_token,
        })
    } else {
        HttpResponse::BadRequest().json(BadRequestError {
            message: "User does not exists.",
            error: "user.login.not_exists",
        })
    }
}
