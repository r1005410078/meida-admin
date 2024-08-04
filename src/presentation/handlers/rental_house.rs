use actix_web::{post, web, HttpResponse};
use tokio::sync::mpsc::Sender;

use crate::{
    domain::houses::{
        command::{
            rental_house_command_save::SaveRentalHouseCommand,
            rental_house_command_save_handler::RentalHouseCommandSaveHandler,
        },
        events::rental_house::SaveRentalHouseEvent,
    },
    infrastructure::repositories::mysql_house_repository::MysqlHouseRepository,
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
