use actix_web::{get, post, web, HttpResponse};
use tokio::sync::mpsc::Sender;

use crate::{
    domain::houses::{
        command::{
            delete_residential_command::DeleteResidentialCommand,
            delete_residential_command_handler::DeleteResidentialCommandHandler,
            new_residential_command::NewResidentialCommand,
            new_residential_command_handler::NewResidentialCommandHandler,
            update_residential_command::UpdateResidentialCommand,
            update_residential_command_handler::UpdateResidentialCommandHandler,
        },
        events::residential::{
            DeleteResidentialEvent, NewResidentialEvent, UpdateResidentialEvent,
        },
    },
    infrastructure::repositories::mysql_residential_repository::MysqlResidentialRepository,
    presentation::service::residential::ResidentialService,
};

#[post("/create")]
async fn create_residential(
    repo: web::Data<MysqlResidentialRepository>,
    sender: web::Data<Sender<NewResidentialEvent>>,
    command: web::Json<NewResidentialCommand>,
) -> HttpResponse {
    let residential = NewResidentialCommandHandler::new(repo.into_inner(), sender.into_inner());

    match residential.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/update")]
async fn update_residential(
    repo: web::Data<MysqlResidentialRepository>,
    sender: web::Data<Sender<UpdateResidentialEvent>>,
    command: web::Json<UpdateResidentialCommand>,
) -> HttpResponse {
    let residential = UpdateResidentialCommandHandler::new(repo.into_inner(), sender.into_inner());

    match residential.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/list")]
async fn list(repo: web::Data<MysqlResidentialRepository>) -> HttpResponse {
    let list = ResidentialService::new(repo.into_inner()).list().await;

    HttpResponse::Ok().json(list)
}

#[get("/get_residential/{community_name}")]
async fn get_residential_by_community_name(
    repo: web::Data<MysqlResidentialRepository>,
    community_name: web::Path<String>,
) -> HttpResponse {
    let result = ResidentialService::new(repo.into_inner())
        .get_residential_by_community_name(community_name.into_inner())
        .await;

    HttpResponse::Ok().json(result)
}

#[post("/delete")]
async fn delete_residential(
    repo: web::Data<MysqlResidentialRepository>,
    sender: web::Data<Sender<DeleteResidentialEvent>>,
    command: web::Json<DeleteResidentialCommand>,
) -> HttpResponse {
    match DeleteResidentialCommandHandler::new(repo.into_inner(), sender.into_inner())
        .handle(command.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
