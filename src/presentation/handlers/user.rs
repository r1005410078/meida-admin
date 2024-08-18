use actix_web::{post, web, HttpResponse};

use crate::{
    infrastructure::repositories::mysql_users_repository::MysqlUsersRepository,
    presentation::{
        dto::users::{LoginDto, SaveUsersDto},
        service::users::UsersService,
    },
};

#[post("/login")]
async fn login(
    repo: web::Data<MysqlUsersRepository>,
    command: web::Json<LoginDto>,
) -> HttpResponse {
    let service = UsersService::new(repo.into_inner());

    match service.login(&command.convert_to_dao()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::InternalServerError()
            .insert_header(("Content-Type", "text/html; charset=utf-8"))
            .body(err.to_string()),
    }
}

#[post("/register")]
async fn register(
    repo: web::Data<MysqlUsersRepository>,
    command: web::Json<SaveUsersDto>,
) -> HttpResponse {
    let service = UsersService::new(repo.into_inner());

    match service.save(&command.convert_to_dao()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
