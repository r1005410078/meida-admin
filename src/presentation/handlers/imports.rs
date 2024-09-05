use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpResponse};

use crate::{
    infrastructure::repositories::{
        entities::imports::QueryImportsPropertiesDto, mysql_house_repository::MysqlHouseRepository,
    },
    presentation::service::imports::{ImportsService, UploadForm},
};

#[post("/properties")]
async fn properties(
    repo: web::Data<MysqlHouseRepository>,
    update_data: MultipartForm<UploadForm>,
) -> HttpResponse {
    let service = ImportsService::new(repo.into_inner());
    match service.upload_file(update_data).await {
        Ok(_) => HttpResponse::Ok().body("ok"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/sync/properties")]
async fn sync_properties(repo: web::Data<MysqlHouseRepository>) -> HttpResponse {
    let service = ImportsService::new(repo.into_inner());
    match service.sync_excel().await {
        Ok(_) => HttpResponse::Ok().body("ok"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/list/properties")]
async fn list_properties(
    repo: web::Data<MysqlHouseRepository>,
    dto: web::Query<QueryImportsPropertiesDto>,
) -> HttpResponse {
    let service = ImportsService::new(repo.into_inner());
    HttpResponse::Ok().json(service.list_imports_properties(dto.into_inner()).await)
}

#[post("/delete/properties")]
async fn delete_imports_properties(
    repo: web::Data<MysqlHouseRepository>,
    ids: web::Json<Vec<String>>,
) -> HttpResponse {
    let service = ImportsService::new(repo.into_inner());
    match service.delete_imports_properties(ids.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("ok"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
