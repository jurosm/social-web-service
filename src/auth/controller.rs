use actix_web::web;

use handler::login;

use super::handler;

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(login);
}
