use actix_web::web;

use crate::presentation::handlers::residential::{
    create_residential, delete_residential, get_residential_by_community_name, list,
    update_residential,
};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/residential")
            .service(create_residential)
            .service(update_residential)
            .service(list)
            .service(get_residential_by_community_name)
            .service(delete_residential),
    );
}
