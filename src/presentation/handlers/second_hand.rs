use actix_web::{get, post, web, HttpResponse};
use tokio::sync::mpsc::Sender;

use crate::{
    domain::houses::{
        command::{
            second_hand_command::{
                SecondHandListedCommand, SecondHandSoldCommand, SecondHandUnlistedCommand,
            },
            second_hand_listed_command_handler::SecondHandListedCommandHandler,
            second_hand_new_command::NewSecondHandCommand,
            second_hand_new_command_handler::NewSecondHandCommandHandler,
            second_hand_sale_command_handler::SecondHandSaleCommandHandler,
            second_hand_unlisted_command_handler::SecondHandUnListedCommandHandler,
            second_hand_update_command::UpdateSecondHandCommand,
            second_hand_update_command_handler::UpdateSecondHandCommandHandler,
        },
        events::second_hand::{
            NewSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent,
            SecondHandUnlistedEvent, UpdateSecondHandEvent,
        },
    },
    infrastructure::repositories::{
        dao::house_second_hand::{QueryHouseSecondHandDto, QueryHouseSecondHandSoldDto},
        mysql_house_repository::MysqlHouseRepository,
    },
    presentation::service::second_hand::SecondHandService,
};

// 新增二手房
#[post("/create")]
async fn create(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<NewSecondHandEvent>>,
    command: web::Json<NewSecondHandCommand>,
) -> HttpResponse {
    let house: NewSecondHandCommandHandler<std::sync::Arc<MysqlHouseRepository>> =
        NewSecondHandCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// 更新二手房
#[post("/update")]
async fn update(
    repo: web::Data<MysqlHouseRepository>,
    sender: web::Data<Sender<UpdateSecondHandEvent>>,
    command: web::Json<UpdateSecondHandCommand>,
) -> HttpResponse {
    let house: UpdateSecondHandCommandHandler<std::sync::Arc<MysqlHouseRepository>> =
        UpdateSecondHandCommandHandler::new(repo.into_inner(), sender.into_inner());

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
    command: web::Json<SecondHandListedCommand>,
) -> HttpResponse {
    let house: SecondHandListedCommandHandler<std::sync::Arc<MysqlHouseRepository>> =
        SecondHandListedCommandHandler::new(repo.into_inner(), sender.into_inner());

    match house.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/list_listed")]
async fn list_listed(
    repo: web::Data<MysqlHouseRepository>,
    query: web::Query<QueryHouseSecondHandDto>,
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
    command: web::Json<SecondHandUnlistedCommand>,
) -> HttpResponse {
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
    command: web::Json<SecondHandSoldCommand>,
) -> HttpResponse {
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
