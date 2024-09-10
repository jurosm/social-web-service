use actix_web::web;

use super::handler::*;

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(create_post_handler)
        .service(update_post_handler)
        .service(get_post_handler)
        .service(get_posts_handler)
        .service(delete_post_handler);
}
