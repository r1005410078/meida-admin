use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde_json::Value;

use crate::{
    infrastructure::repositories::mysql_users_repository::MysqlUsersRepository,
    presentation::{
        dto::users::{LoginDto, QueryUsersDto, SaveUsersDto},
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

    match service.save(&mut command.convert_to_dao()).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/list")]
async fn list(
    repo: web::Data<MysqlUsersRepository>,
    query: web::Json<QueryUsersDto>,
) -> HttpResponse {
    let service = UsersService::new(repo.into_inner());
    let data = service.list(&query.convert_to_dao()).await;
    HttpResponse::Ok().json(data)
}

#[post("/delete/{user_id}")]
async fn delete(repo: web::Data<MysqlUsersRepository>, command: web::Path<String>) -> HttpResponse {
    let service = UsersService::new(repo.into_inner());
    match service.delete_user(&command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/get_user")]
async fn get_user(repo: web::Data<MysqlUsersRepository>, req: HttpRequest) -> HttpResponse {
    if let Some(token) = req.head().headers().get("token") {
        match repo.validate(token.to_str().unwrap_or_default()) {
            Ok(claims) => {
                println!("claims: {:?}", claims);
                let service = UsersService::new(repo.into_inner());
                HttpResponse::Ok().json(service.get_user(&claims.user_id).await)
            }
            Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
        }
    } else {
        HttpResponse::Ok().json(Value::Null)
    }
}
