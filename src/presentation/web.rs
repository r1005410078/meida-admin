use std::env;
use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use actix_web::get;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpServer};
use log::info;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::domain::houses::events::rental_house::DeleteRentalHouseEvent;
use crate::domain::houses::events::second_hand::DeleteSecondHandEvent;
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
        mysql_house_repository::MysqlHouseRepository, mysql_users_repository::MysqlUsersRepository,
    },
    presentation::{
        events::{
            house_event::HouseEventHandler, rental_house::RentalHouseHandler,
            residential_event::ResidentialEventHandler, second_hand::SecondHandEventHandler,
        },
        routes,
    },
};

#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> Result<NamedFile, Error> {
    let mut path: PathBuf = req.match_info().query("filename").parse().unwrap();

    // 如果只有目录，默认返回当前目录下的 index.html 文件
    if path.extension() == None {
        path.push("index.html");
    }

    let web_root_dir = env::var("WEB_ROOT_DIR").unwrap_or("/opt/www/meida/dist".to_string());
    let static_path = Path::new(&web_root_dir);

    let file = NamedFile::open(static_path.join(path))?;

    Ok(file.use_last_modified(true).use_etag(true))
}

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

    let second_hand_delete_sender = web::Data::new(
        EventChannel::<DeleteSecondHandEvent>::new(SecondHandEventHandler::new(
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

    // 删除
    let delete_rental_house = web::Data::new(
        EventChannel::<DeleteRentalHouseEvent>::new(RentalHouseHandler::new(
            house.clone().into_inner(),
        ))
        .sender,
    );

    // user mysql
    let users = web::Data::new(MysqlUsersRepository::new());

    info!("Web server Starting...");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    let cert_dir_path =
        env::var("CERTIFICATE_DIR").unwrap_or("/opt/www/meida/certificate".to_string());
    let cert_dir = Path::new(&cert_dir_path);

    println!("cert_dir: {:?}", cert_dir);

    builder
        .set_private_key_file(cert_dir.join("rongts.tech.key"), SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(cert_dir.join("rongts.tech.pem"))
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(users.clone())
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
            .app_data(second_hand_delete_sender.clone())
            .app_data(save_rental_house_sender.clone())
            .app_data(rental_house_listed_event.clone())
            .app_data(rental_house_unlisted_event.clone())
            .app_data(rental_house_sold_event.clone())
            .app_data(delete_rental_house.clone())
            .wrap(Logger::default())
            .configure(routes::residential_routes::routes)
            .configure(routes::house::routes)
            .configure(routes::qiliu::routes)
            .configure(routes::second_hand::routes)
            .configure(routes::rental_house::routes)
            .configure(routes::user::routes)
            .service(index)
    })
    .bind_openssl("0.0.0.0:443", builder)?
    .bind("0.0.0.0:80")?
    .run()
    .await
}
