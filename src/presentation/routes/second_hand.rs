use actix_web::{web, HttpResponse};

use crate::presentation::{
    guard::users::UserGuard,
    handlers::second_hand::{
        delete_second_hand, get_by_house_id, list_listed, list_sold, listed, save, sold, unlisted,
    },
};

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/api/v1/second_hand_house")
                .guard(UserGuard)
                .service(save)
                .service(listed)
                .service(sold)
                .service(list_listed)
                .service(get_by_house_id)
                .service(unlisted)
                .service(list_sold)
                .service(delete_second_hand),
        )
        .default_service(web::route().to(|| HttpResponse::Unauthorized()));
}
