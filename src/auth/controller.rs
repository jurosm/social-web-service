use actix_web::web;

use handler::login;
use handler::refresh;

use super::handler;

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(login);
    conf.service(refresh);
}
