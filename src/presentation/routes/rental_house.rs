use actix_web::web;

use crate::presentation::handlers::rental_house::save;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1/rental_house").service(save));
}
