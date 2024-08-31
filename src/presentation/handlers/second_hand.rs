use actix_web::{get, post, web, HttpRequest, HttpResponse};
use tokio::sync::mpsc::Sender;

use crate::{
    common::jwt::Claims,
    domain::houses::{
        command::{
            second_hand_command::{
                DeleteSecondHandCommand, SecondHandListedCommand, SecondHandSoldCommand,
                SecondHandUnlistedCommand,
            },
            second_hand_delete_command_handler::DeleteSecondHandCommandHandler,
            second_hand_listed_command_handler::SecondHandListedCommandHandler,
            second_hand_sale_command_handler::SecondHandSaleCommandHandler,
            second_hand_save_command::SaveSecondHandCommand,
            second_hand_save_command_handler::SaveSecondHandCommandHandler,
            second_hand_unlisted_command_handler::SecondHandUnListedCommandHandler,
        },
        events::second_hand::{
            DeleteSecondHandEvent, SaveSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent,
            SecondHandUnlistedEvent,
        },
    },
    infrastructure::repositories::{
        dao::house_second_hand::{QueryHouseSecondHandDto, QueryHouseSecondHandSoldDto},
        mysql_house_repository::MysqlHouseRepository,
    },
    presentation::service::second_hand::SecondHandService,
};

// 保存二手房
#[post("/save")]
async fn save(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<SaveSecondHandEvent>>,
    mut command: web::Json<SaveSecondHandCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());
    command.created_by.replace(user.username.clone());

    let house: SaveSecondHandCommandHandler<std::sync::Arc<MysqlHouseRepository>> =
        SaveSecondHandCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// 保存上架数据
#[post("/listed")]
async fn listed(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<SecondHandListedEvent>>,
    mut command: web::Json<SecondHandListedCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());
    command.created_by.replace(user.username.clone());

    let house: SecondHandListedCommandHandler<std::sync::Arc<MysqlHouseRepository>> =
        SecondHandListedCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/list_listed")]
async fn list_listed(
    repo: web::Data<MysqlHouseRepository>,
    query: web::Json<QueryHouseSecondHandDto>,
) -> HttpResponse {
    let list = SecondHandService::new(repo.into_inner())
        .list_listed(query.into_inner())
        .await;

    HttpResponse::Ok().json(list)
}

// 根据id获取二手房信息
#[get("/get_by_house_id/{id}")]
async fn get_by_house_id(
    repo: web::Data<MysqlHouseRepository>,
    house_id: web::Path<String>,
) -> HttpResponse {
    let house_id = house_id.into_inner();
    let second_hand = SecondHandService::new(repo.into_inner())
        .house_second_hand_by_house_id(house_id)
        .await;

    HttpResponse::Ok().json(second_hand)
}

// 保存下架数据
#[post("/unlisted")]
async fn unlisted(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<SecondHandUnlistedEvent>>,
    mut command: web::Json<SecondHandUnlistedCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());
    command.created_by.replace(user.username.clone());

    let house: SecondHandUnListedCommandHandler<std::sync::Arc<MysqlHouseRepository>> =
        SecondHandUnListedCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// 卖出
#[post("/sold")]
async fn sold(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<SecondHandSoldEvent>>,
    mut command: web::Json<SecondHandSoldCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());
    command.created_by.replace(user.username.clone());

    let house: SecondHandSaleCommandHandler<std::sync::Arc<MysqlHouseRepository>> =
        SecondHandSaleCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// 获取卖出的数据
#[get("/list_sold")]
async fn list_sold(
    repo: web::Data<MysqlHouseRepository>,
    query: web::Query<QueryHouseSecondHandSoldDto>,
) -> HttpResponse {
    let list = SecondHandService::new(repo.into_inner())
        .list_sold(query.into_inner())
        .await;

    HttpResponse::Ok().json(list)
}

// 删除二手房
#[post("/delete")]
async fn delete_second_hand(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<DeleteSecondHandEvent>>,
    mut command: web::Json<DeleteSecondHandCommand>,
    req: HttpRequest,
) -> HttpResponse {
    let user_token = req.headers().get("token").unwrap().to_str().unwrap();
    let user = Claims::validate(user_token).unwrap();

    command.updated_by.replace(user.username.clone());

    let delete_second_handler =
        DeleteSecondHandCommandHandler::new(repo.into_inner(), sender.into_inner());

    match delete_second_handler.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
