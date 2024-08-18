use crate::presentation::{
    guard::users::UserGuard,
    handlers::rental_house::{detail, list, list_sold, listed, save, sold, unlisted},
};
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/rental_house")
            .guard(UserGuard)
            .service(save)
            .service(list)
            .service(list_sold)
            .service(listed)
            .service(unlisted)
            .service(detail)
            .service(sold),
    );
}
