use actix_web::web;

use crate::presentation::handlers::rental_house::{list_listed, list_sold, save};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/rental_house")
            .service(save)
            .service(list_listed)
            .service(list_sold),
    );
}
