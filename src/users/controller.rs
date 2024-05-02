use actix_web::web;

use handler::{
    create_user_handler, delete_user_handler, get_user_handler, get_users_handler,
    update_user_handler,
};

use super::handler;

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(create_user_handler)
        .service(update_user_handler)
        .service(get_user_handler)
        .service(delete_user_handler)
        .service(get_users_handler);
}
