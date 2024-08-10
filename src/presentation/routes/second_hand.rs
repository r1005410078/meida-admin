use actix_web::web;

use crate::presentation::handlers::second_hand::{
    get_by_house_id, list_listed, list_sold, listed, save, sold, unlisted,
};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/second_hand_house")
            .service(save)
            .service(listed)
            .service(sold)
            .service(list_listed)
            .service(get_by_house_id)
            .service(unlisted)
            .service(list_sold),
    );
}
