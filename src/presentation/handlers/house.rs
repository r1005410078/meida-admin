use actix_web::{get, post, web, HttpResponse};
use tokio::sync::mpsc::Sender;

use crate::{
    domain::houses::{
        command::{
            new_house_command::NewHouseCommand, new_house_command_handler::NewHouseCommandHandler,
            update_house_command::UpdateHouseCommand,
            update_house_command_handler::UpdateHouseCommandHandler,
        },
        events::house::{NewHouseEvent, UpdateHouseEvent},
    },
    infrastructure::repositories::mysql_house_repository::MysqlHouseRepository,
    presentation::service::house::HouseService,
};

#[post("/create")]
async fn create_house(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<NewHouseEvent>>,
    command: web::Json<NewHouseCommand>,
) -> HttpResponse {
    let house = NewHouseCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/update")]
async fn update_house(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<UpdateHouseEvent>>,
    command: web::Json<UpdateHouseCommand>,
) -> HttpResponse {
    let house = UpdateHouseCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/list")]
async fn list(repo: web::Data<MysqlHouseRepository>) -> HttpResponse {
    let list = HouseService::new(repo.into_inner()).list().await;

    HttpResponse::Ok().json(list)
}
