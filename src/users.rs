use actix_web::web;

use self::handler::{
    create_user_handler, delete_user_handler, get_user_handler, get_users_handler,
    update_user_handler,
};

pub mod handler;

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(create_user_handler)
        .service(update_user_handler)
        .service(get_user_handler)
        .service(delete_user_handler)
        .service(get_users_handler);
}
