use crate::presentation::{
    handlers::response::{ApiError, Response},
    service::qiliu,
};
use actix_web::{get, web, Responder, Result};

#[get("/get_token/{file_name}")]
async fn get_qiniu_token(file_name: web::Path<String>) -> Result<impl Responder> {
    let token = qiliu::get_upload_token(&file_name);
    Ok(Response::ok(token))
}

#[get("/get_upload_url/{file_name}")]
async fn get_upload_url(file_name: web::Path<String>) -> Result<impl Responder> {
    let token = qiliu::get_upload_url(&file_name).map_err(|e| ApiError::ErrMsg(e.to_string()))?;
    Ok(Response::ok(token.to_string()))
}
