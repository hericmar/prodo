use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use uuid::Uuid;
use crate::core::models::task::{CreateTask, UpdateTask};
use crate::core::services::task::TaskService;
use crate::prelude::*;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub summary: String,
}

pub async fn create_task_handler(
    user: Identity,
    task_service: web::Data<dyn TaskService>,
    payload: web::Json<CreateTaskRequest>
) -> Result<HttpResponse> {
    let new_task = CreateTask {
        summary: payload.summary.clone(),
        author_uid: user.id()?.parse().unwrap()
    };
    let result = task_service.create(new_task).await?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn read_tasks_handler(
    user: Identity,
    task_service: web::Data<dyn TaskService>
) -> Result<HttpResponse> {
    let author_uid: Uuid = user.id()?.parse().unwrap();
    let result = task_service.list(author_uid).await?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_task_handler(
    task_id: web::Path<Uuid>,
    user: Identity,
    task_service: web::Data<dyn TaskService>,
    payload: web::Json<UpdateTask>
) -> Result<HttpResponse> {
    let task = task_service.update(task_id.into_inner(), payload.into_inner()).await?;

    Ok(HttpResponse::Ok().json(task))
}

pub async fn delete_task_handler(
    task_id: web::Path<Uuid>,
    user: Identity,
    task_service: web::Data<dyn TaskService>
) -> Result<HttpResponse> {
    task_service.delete(task_id.into_inner()).await?;

    Ok(HttpResponse::Ok().finish())
}
