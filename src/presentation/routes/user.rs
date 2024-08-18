use actix_web::web;

use crate::presentation::handlers::user::{login, register};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1/users").service(login).service(register));
}
