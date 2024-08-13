use actix_web::{get, post, web, HttpResponse};
use serde_json::json;
use tokio::sync::mpsc::Sender;

use crate::{
    domain::houses::{
        command::{
            delete_house_command::DeleteHouseCommand,
            delete_house_command_handler::DeleteHouseCommandHandler,
            house_save_command::SaveHouseCommand,
            house_save_command_handler::HouseSaveCommandHandler,
        },
        events::house::{DeleteHouseEvent, SaveHouseEvent},
    },
    infrastructure::repositories::{
        dao::house::QueryHouseDao, mysql_house_repository::MysqlHouseRepository,
    },
    presentation::service::house::HouseService,
};

#[post("/save")]
async fn save_house(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<SaveHouseEvent>>,
    command: web::Json<SaveHouseCommand>,
) -> HttpResponse {
    let house = HouseSaveCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(house_id) => HttpResponse::Ok().json(json!({ "house_id": house_id })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/delete")]
async fn delete_house(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<DeleteHouseEvent>>,
    command: web::Json<DeleteHouseCommand>,
) -> HttpResponse {
    let house = DeleteHouseCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/list")]
async fn list(
    repo: web::Data<MysqlHouseRepository>,
    query: web::Query<QueryHouseDao>,
) -> HttpResponse {
    let list = HouseService::new(repo.into_inner())
        .list(query.into_inner())
        .await;

    HttpResponse::Ok().json(list)
}

#[get["/get_house/{house_id}"]]
async fn get_house_by_house_id(
    repo: web::Data<MysqlHouseRepository>,
    house_id: web::Path<String>,
) -> HttpResponse {
    let result = HouseService::new(repo.into_inner())
        .get_by_house_id(house_id.into_inner())
        .await;

    HttpResponse::Ok().json(result)
}

#[get["/list_by_owner_name/{input_owner_name}"]]
async fn list_by_owner_name(
    repo: web::Data<MysqlHouseRepository>,
    input_owner_name: web::Path<String>,
) -> HttpResponse {
    let result = HouseService::new(repo.into_inner())
        .list_by_owner_name(input_owner_name.into_inner())
        .await;

    HttpResponse::Ok().json(result)
}
