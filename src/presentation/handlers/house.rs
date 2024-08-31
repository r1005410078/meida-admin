use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde_json::json;
use tokio::sync::mpsc::Sender;

use crate::{
    common::jwt::Claims,
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
    mut command: web::Json<SaveHouseCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());
    command.created_by.replace(user.username.clone());

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

#[post("/list")]
async fn list(
    repo: web::Data<MysqlHouseRepository>,
    query: web::Json<QueryHouseDao>,
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
