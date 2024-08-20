use actix_web::web;

use crate::presentation::{
    guard::users::UserGuard,
    handlers::user::{delete, get_user, list, login, register},
};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1/auth").service(login).service(get_user));

    config.service(
        web::scope("/api/v1/users")
            .guard(UserGuard)
            .service(list)
            .service(register)
            .service(delete),
    );
}
