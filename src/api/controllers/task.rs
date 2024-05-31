use actix_web::{HttpResponse, web};
use uuid::Uuid;
use crate::core::services::task::TaskService;
use crate::prelude::*;

pub async fn create_task_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn read_tasks_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn update_task_handler(
    task_id: web::Path<Uuid>,
    task_service: web::Data<dyn TaskService>
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn delete_task_handler(
    task_id: web::Path<Uuid>,
    task_service: web::Data<dyn TaskService>
) -> Result<HttpResponse> {
    task_service.delete(task_id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
