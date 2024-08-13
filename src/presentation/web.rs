use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

use crate::{
    common::event_channel::EventChannel,
    domain::houses::events::{
        house::{DeleteHouseEvent, SaveHouseEvent},
        rental_house::{
            RentalHouseListedEvent, RentalHouseSoldEvent, RentalHouseUnListedEvent,
            SaveRentalHouseEvent,
        },
        residential::{DeleteResidentialEvent, SaveCommunityEvent},
        second_hand::{
            SaveSecondHandEvent, SecondHandListedEvent, SecondHandSoldEvent,
            SecondHandUnlistedEvent,
        },
    },
    infrastructure::repositories::{
        mysql_community_repository::MysqlResidentialRepository,
        mysql_house_repository::MysqlHouseRepository,
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
    let save_residential_sender = web::Data::new(
        EventChannel::<SaveCommunityEvent>::new(ResidentialEventHandler::new(
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
    let update_house_sender = web::Data::new(
        EventChannel::<SaveHouseEvent>::new(HouseEventHandler::new(house.clone().into_inner()))
            .sender,
    );

    let delete_house_sender = web::Data::new(
        EventChannel::<DeleteHouseEvent>::new(HouseEventHandler::new(house.clone().into_inner()))
            .sender,
    );

    // 二手房事件
    let save_second_hand_sender = web::Data::new(
        EventChannel::<SaveSecondHandEvent>::new(SecondHandEventHandler::new(
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

    // 上架
    let rental_house_listed_event = web::Data::new(
        EventChannel::<RentalHouseListedEvent>::new(RentalHouseHandler::new(
            house.clone().into_inner(),
        ))
        .sender,
    );

    // 下架
    let rental_house_unlisted_event = web::Data::new(
        EventChannel::<RentalHouseUnListedEvent>::new(RentalHouseHandler::new(
            house.clone().into_inner(),
        ))
        .sender,
    );

    // 出售
    let rental_house_sold_event = web::Data::new(
        EventChannel::<RentalHouseSoldEvent>::new(RentalHouseHandler::new(
            house.clone().into_inner(),
        ))
        .sender,
    );

    info!("Web server Starting...");

    HttpServer::new(move || {
        App::new()
            .app_data(residential.clone())
            .app_data(save_residential_sender.clone())
            .app_data(delete_residential_sender.clone())
            .app_data(house.clone())
            .app_data(update_house_sender.clone())
            .app_data(delete_house_sender.clone())
            .app_data(second_hand_listed_sender.clone())
            .app_data(second_hand_unlisted_sender.clone())
            .app_data(second_hand_scale_sender.clone())
            .app_data(save_second_hand_sender.clone())
            .app_data(save_rental_house_sender.clone())
            .app_data(rental_house_listed_event.clone())
            .app_data(rental_house_unlisted_event.clone())
            .app_data(rental_house_sold_event.clone())
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
