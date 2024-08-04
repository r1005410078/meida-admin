use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

use crate::{
    common::event_channel::EventChannel,
    domain::houses::events::{
        house::{DeleteHouseEvent, NewHouseEvent, UpdateHouseEvent},
        rental_house::SaveRentalHouseEvent,
        residential::{DeleteResidentialEvent, NewResidentialEvent, UpdateResidentialEvent},
        second_hand::{
            NewSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent,
            SecondHandUnlistedEvent, UpdateSecondHandEvent,
        },
    },
    infrastructure::repositories::{
        mysql_house_repository::MysqlHouseRepository,
        mysql_residential_repository::MysqlResidentialRepository,
    },
    presentation::{
        events::{
            house_event::HouseEventHandler, rental_house::RentalHouseHandler,
            residential_event::ResidentialEventHandler, second_hand::SecondHandEventHandler,
        },
        routes,
    },
};

pub async fn run() -> std::io::Result<()> {
    let residential = web::Data::new(MysqlResidentialRepository::new());

    // 小区事件
    let new_residential_sender = web::Data::new(
        EventChannel::<NewResidentialEvent>::new(ResidentialEventHandler::new(
            residential.clone().into_inner(),
        ))
        .sender,
    );

    let update_residential_sender = web::Data::new(
        EventChannel::<UpdateResidentialEvent>::new(ResidentialEventHandler::new(
            residential.clone().into_inner(),
        ))
        .sender,
    );

    let delete_residential_sender = web::Data::new(
        EventChannel::<DeleteResidentialEvent>::new(ResidentialEventHandler::new(
            residential.clone().into_inner(),
        ))
        .sender,
    );

    // 房屋事件
    let house = web::Data::new(MysqlHouseRepository::new());
    let new_house_sender: web::Data<tokio::sync::mpsc::Sender<NewHouseEvent>> = web::Data::new(
        EventChannel::<NewHouseEvent>::new(HouseEventHandler::new(house.clone().into_inner()))
            .sender,
    );

    let update_house_sender = web::Data::new(
        EventChannel::<UpdateHouseEvent>::new(HouseEventHandler::new(house.clone().into_inner()))
            .sender,
    );

    let delete_house_sender = web::Data::new(
        EventChannel::<DeleteHouseEvent>::new(HouseEventHandler::new(house.clone().into_inner()))
            .sender,
    );

    // 二手房事件
    let create_second_hand_sender = web::Data::new(
        EventChannel::<NewSecondHandEvent>::new(SecondHandEventHandler::new(
            house.clone().into_inner(),
        ))
        .sender,
    );

    let update_second_hand_sender = web::Data::new(
        EventChannel::<UpdateSecondHandEvent>::new(SecondHandEventHandler::new(
            house.clone().into_inner(),
        ))
        .sender,
    );

    let second_hand_listed_sender = web::Data::new(
        EventChannel::<SecondHandListedEvent>::new(SecondHandEventHandler::new(
            house.clone().into_inner(),
        ))
        .sender,
    );

    let second_hand_unlisted_sender = web::Data::new(
        EventChannel::<SecondHandUnlistedEvent>::new(SecondHandEventHandler::new(
            house.clone().into_inner(),
        ))
        .sender,
    );

    let second_hand_scale_sender = web::Data::new(
        EventChannel::<SecondHandSoldEvent>::new(SecondHandEventHandler::new(
            house.clone().into_inner(),
        ))
        .sender,
    );

    // 租房
    let save_rental_house_sender = web::Data::new(
        EventChannel::<SaveRentalHouseEvent>::new(RentalHouseHandler::new(
            house.clone().into_inner(),
        ))
        .sender,
    );

    info!("Web server Starting...");

    HttpServer::new(move || {
        App::new()
            .app_data(residential.clone())
            .app_data(new_residential_sender.clone())
            .app_data(update_residential_sender.clone())
            .app_data(delete_residential_sender.clone())
            .app_data(house.clone())
            .app_data(new_house_sender.clone())
            .app_data(update_house_sender.clone())
            .app_data(delete_house_sender.clone())
            .app_data(second_hand_listed_sender.clone())
            .app_data(second_hand_unlisted_sender.clone())
            .app_data(second_hand_scale_sender.clone())
            .app_data(create_second_hand_sender.clone())
            .app_data(save_rental_house_sender.clone())
            .app_data(save_rental_house_sender.clone())
            .wrap(Logger::default())
            .configure(routes::residential_routes::routes)
            .configure(routes::house::routes)
            .configure(routes::qiliu::routes)
            .configure(routes::second_hand::routes)
            .configure(routes::rental_house::routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
