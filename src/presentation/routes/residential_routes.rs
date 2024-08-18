use actix_web::{web, HttpResponse};

use crate::presentation::{
    guard::users::UserGuard,
    handlers::residential::{
        delete_residential, get_community_names, get_residential_by_community_name, list,
        save_community,
    },
};

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/api/v1/residential")
                .guard(UserGuard)
                .service(save_community)
                .service(list)
                .service(get_residential_by_community_name)
                .service(delete_residential)
                .service(get_community_names),
        )
        .default_service(web::route().to(|| HttpResponse::Unauthorized()));
}
