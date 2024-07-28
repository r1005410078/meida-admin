use actix_web::web;

use crate::presentation::handlers::residential::{create_residential, list, update_residential};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/residential")
            .service(create_residential)
            .service(update_residential)
            .service(list),
    );
}
