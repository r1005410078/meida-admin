use actix_web::{web, HttpResponse};

use crate::presentation::{
    guard::users::UserGuard,
    handlers::house::{delete_house, get_house_by_house_id, list, list_by_owner_name, save_house},
};

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/api/v1/house")
                .guard(UserGuard)
                .service(save_house)
                .service(list)
                .service(get_house_by_house_id)
                .service(delete_house)
                .service(list_by_owner_name),
        )
        .default_service(web::route().to(|| HttpResponse::Unauthorized()));
}
