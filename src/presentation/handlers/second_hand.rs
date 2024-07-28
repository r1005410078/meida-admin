use actix_web::{post, web, HttpResponse};
use tokio::sync::mpsc::Sender;

use crate::{
    domain::houses::{
        command::{
            second_hand_command::{
                SecondHandListedCommand, SecondHandSaleCommand, SecondHandUnlistedCommand,
            },
            second_hand_listed_command_handler::SecondHandListedCommandHandler,
            second_hand_sale_command_handler::SecondHandSaleCommandHandler,
            second_hand_unlisted_command_handler::SecondHandUnListedCommandHandler,
        },
        events::second_hand::{
            SecondHandListedEvent, SecondHandSaleEvent, SecondHandUnlistedEvent,
        },
    },
    infrastructure::repositories::mysql_house_repository::MysqlHouseRepository,
};

#[post("/listed")]
async fn listed(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<SecondHandListedEvent>>,
    command: web::Json<SecondHandListedCommand>,
) -> HttpResponse {
    let house = SecondHandListedCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/unlisted")]
async fn unlisted(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<SecondHandUnlistedEvent>>,
    command: web::Json<SecondHandUnlistedCommand>,
) -> HttpResponse {
    let house = SecondHandUnListedCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/sale")]
async fn sale(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<SecondHandSaleEvent>>,
    command: web::Json<SecondHandSaleCommand>,
) -> HttpResponse {
    let house = SecondHandSaleCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
