use actix_web::{get, post, web, HttpResponse};
use tokio::sync::mpsc::Sender;

use crate::{
    domain::houses::{
        command::{
            community_save_command::CommunitySaveCommand,
            community_save_command_handler::CommunitySaveCommandHandler,
            delete_residential_command::DeleteResidentialCommand,
            delete_residential_command_handler::DeleteResidentialCommandHandler,
        },
        events::residential::{DeleteResidentialEvent, SaveCommunityEvent},
    },
    infrastructure::repositories::{
        dao::community::QueryCommunityDao, mysql_community_repository::MysqlResidentialRepository,
    },
    presentation::service::residential::ResidentialService,
};

#[post("/save")]
async fn save_community(
    repo: web::Data<MysqlResidentialRepository>,
    sender: web::Data<Sender<SaveCommunityEvent>>,
    command: web::Json<CommunitySaveCommand>,
) -> HttpResponse {
    let residential = CommunitySaveCommandHandler::new(repo.into_inner(), sender.into_inner());

    match residential.handle(command.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/list")]
async fn list(
    repo: web::Data<MysqlResidentialRepository>,
    query: web::Query<QueryCommunityDao>,
) -> HttpResponse {
    let list = ResidentialService::new(repo.into_inner())
        .list(query.into_inner())
        .await;

    HttpResponse::Ok().json(list)
}

#[get("/get_community_names")]
async fn get_community_names(repo: web::Data<MysqlResidentialRepository>) -> HttpResponse {
    let names = ResidentialService::new(repo.into_inner())
        .get_community_names()
        .await;
    HttpResponse::Ok().json(names)
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
