use actix_web::web;

use crate::presentation::handlers::qiliu::{get_qiniu_token, get_upload_url};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/qiniu")
            .service(get_qiniu_token)
            .service(get_upload_url),
    );
}
