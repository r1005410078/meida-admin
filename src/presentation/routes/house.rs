use actix_web::web;

use crate::presentation::handlers::house::{
    create_house, delete_house, get_house_by_house_id, list, update_house,
};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/house")
            .service(create_house)
            .service(update_house)
            .service(list)
            .service(get_house_by_house_id)
            .service(delete_house),
    );
}
