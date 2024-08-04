use actix_web::{get, post, web, HttpResponse};
use tokio::sync::mpsc::Sender;

use crate::{
    domain::houses::{
        command::{
            rental_house_command_save::SaveRentalHouseCommand,
            rental_house_command_save_handler::RentalHouseCommandSaveHandler,
        },
        events::rental_house::SaveRentalHouseEvent,
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
    command: web::Json<SaveRentalHouseCommand>,
) -> HttpResponse {
    let house = RentalHouseCommandSaveHandler::new(repo.into_inner(), sender.into_inner());
    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/list_listed")]
async fn list_listed(
    repo: web::Data<MysqlHouseRepository>,
    query: web::Query<QueryRentalHouseListedDto>,
) -> HttpResponse {
    let list = RentalHouseService::new(repo.into_inner())
        .list_listed(query.into_inner())
        .await;

    HttpResponse::Ok().json(list)
}

#[get("/list_sold")]
async fn list_sold(
    repo: web::Data<MysqlHouseRepository>,
    query: web::Query<QueryRentalHouseSoldDto>,
) -> HttpResponse {
    let list = RentalHouseService::new(repo.into_inner())
        .list_sold(query.into_inner())
        .await;

    HttpResponse::Ok().json(list)
}
