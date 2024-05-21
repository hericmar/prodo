use actix_web::HttpResponse;
use crate::prelude::*;

pub async fn create_task_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn read_tasks_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn update_task_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_task_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
