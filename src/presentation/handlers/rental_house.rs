use actix_web::{get, post, web, HttpRequest, HttpResponse};
use tokio::sync::mpsc::Sender;

use crate::{
    common::jwt::Claims,
    domain::houses::{
        command::{
            rental_house_delete_command::DeleteRentalHouseCommand,
            rental_house_delete_command_handler::DeleteRentalHouseCommandHandler,
            rental_house_listed_command::RentalHouseListedCommand,
            rental_house_listed_command_handler::RentalHouseListedCommandHandler,
            rental_house_save_command::SaveRentalHouseCommand,
            rental_house_save_command_handler::RentalHouseCommandSaveHandler,
            rental_house_sold_command::RentalHouseSoldCommand,
            rental_house_sold_command_handler::RentalHouseSoldCommandHandler,
            rental_house_unlisted_command::RentalHouseUnListedCommand,
            rental_house_unlisted_command_handler::RentalHouseUnListedCommandHandler,
        },
        events::rental_house::{
            DeleteRentalHouseEvent, RentalHouseListedEvent, RentalHouseSoldEvent,
            RentalHouseUnListedEvent, SaveRentalHouseEvent,
        },
    },
    infrastructure::repositories::{
        dao::rental_house::{QueryRentalHouseListedDto, QueryRentalHouseSoldDto},
        mysql_house_repository::MysqlHouseRepository,
    },
    presentation::service::rental_house::RentalHouseService,
};

#[post("/save")]
async fn save(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<SaveRentalHouseEvent>>,
    mut command: web::Json<SaveRentalHouseCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());
    command.created_by.replace(user.username.clone());

    let house = RentalHouseCommandSaveHandler::new(repo.into_inner(), sender.into_inner());
    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/list")]
async fn list(
    repo: web::Data<MysqlHouseRepository>,
    query: web::Json<QueryRentalHouseListedDto>,
) -> HttpResponse {
    let list = RentalHouseService::new(repo.into_inner())
        .list(query.into_inner())
        .await;

    HttpResponse::Ok().json(list)
}

#[get["/detail/{id}"]]
async fn detail(repo: web::Data<MysqlHouseRepository>, query: web::Path<String>) -> HttpResponse {
    let detail = RentalHouseService::new(repo.into_inner())
        .detail(query.into_inner())
        .await;

    HttpResponse::Ok().json(detail)
}

#[get("/list_sold")]
async fn list_sold(
    repo: web::Data<MysqlHouseRepository>,
    query: web::Query<QueryRentalHouseSoldDto>,
) -> HttpResponse {
    let list_sold = RentalHouseService::new(repo.into_inner())
        .list_sold(query.into_inner())
        .await;

    HttpResponse::Ok().json(list_sold)
}

#[post("/listed")]
async fn listed(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<RentalHouseListedEvent>>,
    mut command: web::Json<RentalHouseListedCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());
    command.created_by.replace(user.username.clone());

    let house = RentalHouseListedCommandHandler::new(repo.into_inner(), sender.into_inner());
    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/unlisted")]
async fn unlisted(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<RentalHouseUnListedEvent>>,
    mut command: web::Json<RentalHouseUnListedCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());
    command.created_by.replace(user.username.clone());

    let house = RentalHouseUnListedCommandHandler::new(repo.into_inner(), sender.into_inner());
    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/sold")]
async fn sold(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<RentalHouseSoldEvent>>,
    mut command: web::Json<RentalHouseSoldCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());
    command.created_by.replace(user.username.clone());

    let house = RentalHouseSoldCommandHandler::new(repo.into_inner(), sender.into_inner());
    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/delete")]
async fn delete(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<DeleteRentalHouseEvent>>,
    mut command: web::Json<DeleteRentalHouseCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());

    let delete = DeleteRentalHouseCommandHandler::new(repo.into_inner(), sender.into_inner());

    match delete.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
