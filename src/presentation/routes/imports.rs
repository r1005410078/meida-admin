use actix_web::{web, HttpResponse};

use crate::presentation::{
    guard::users::UserGuard,
    handlers::imports::{delete_imports_properties, list_properties, properties, sync_properties},
};

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/api/v1/imports")
                .guard(UserGuard)
                .service(properties)
                .service(list_properties)
                .service(sync_properties)
                .service(delete_imports_properties),
        )
        .default_service(web::route().to(|| HttpResponse::Unauthorized()));
}
