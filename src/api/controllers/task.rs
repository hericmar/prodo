use crate::core::models::task::{CreateTask, UpdateTask, UpdateTaskList};
use crate::core::services::task::{TaskListService, TaskService};
use crate::error::{Error, ErrorType};
use crate::prelude::*;
use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub summary: String,
}

pub async fn read_task_lists_handler(
    user: Identity,
    task_list_service: web::Data<dyn TaskListService>,
) -> Result<HttpResponse> {
    let author_uid: Uuid = user.id()?.parse().unwrap();
    let result = task_list_service.list(author_uid).await?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn create_task_handler(
    user: Identity,
    list_uid: web::Path<Uuid>,
    task_service: web::Data<dyn TaskService>,
    payload: web::Json<CreateTaskRequest>,
) -> Result<HttpResponse> {
    let new_task = CreateTask {
        summary: payload.summary.clone(),
        author_uid: user.id()?.parse().unwrap(),
    };
    let result = task_service
        .create(new_task, Some(list_uid.into_inner()))
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn read_tasks_handler(
    user: Identity,
    task_service: web::Data<dyn TaskService>,
) -> Result<HttpResponse> {
    let author_uid: Uuid = user.id()?.parse().unwrap();
    let result = task_service.list(author_uid).await?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_task_handler(
    task_uid: web::Path<Uuid>,
    user: Identity,
    task_service: web::Data<dyn TaskService>,
    payload: web::Json<UpdateTask>,
) -> Result<HttpResponse> {
    let task_uid = task_uid.into_inner();
    let old_task = task_service.get(task_uid.clone()).await?;
    if old_task.author_uid != user.id()?.parse().unwrap() {
        return Err(Error::new("unauthorized", ErrorType::Unauthorized));
    }

    let task = task_service.update(task_uid, payload.into_inner()).await?;

    Ok(HttpResponse::Ok().json(task))
}

pub async fn delete_task_handler(
    task_uid: web::Path<Uuid>,
    user: Identity,
    task_service: web::Data<dyn TaskService>,
) -> Result<HttpResponse> {
    let task_uid = task_uid.into_inner();
    let task = task_service.get(task_uid.clone()).await?;
    if task.author_uid != user.id()?.parse().unwrap() {
        return Err(Error::new("unauthorized", ErrorType::Unauthorized));
    }

    task_service.delete(task_uid).await?;

    Ok(HttpResponse::Ok().finish())
}

//

#[derive(Deserialize)]
pub struct UpdateTaskListRequest {
    pub name: String,
}

pub async fn update_task_list_handler(
    list_uid: web::Path<Uuid>,
    user: Identity,
    task_list_service: web::Data<dyn TaskListService>,
    payload: web::Json<UpdateTaskListRequest>,
) -> Result<HttpResponse> {
    let list_uid = list_uid.into_inner();
    let task_list = task_list_service.get(list_uid.clone()).await?;
    if task_list.author_uid != user.id()?.parse().unwrap() {
        return Err(Error::new("unauthorized", ErrorType::Unauthorized));
    }
    let task_list = task_list_service
        .update(
            list_uid,
            UpdateTaskList {
                name: Some(payload.name.clone()),
                tasks: None,
            },
        )
        .await?;

    Ok(HttpResponse::Ok().json(task_list))
}

#[derive(Deserialize)]
pub struct UpdateTaskPositionRequest {
    pub position: i32,
}

#[derive(Deserialize)]
pub struct UpdateTaskPositionParams {
    pub task_uid: Uuid,
    pub list_uid: Uuid,
}

pub async fn update_task_position_handler(
    params: web::Path<UpdateTaskPositionParams>,
    user: Identity,
    payload: web::Json<UpdateTaskPositionRequest>,
    task_list_service: web::Data<dyn TaskListService>,
) -> Result<HttpResponse> {
    let task_list = task_list_service.get(params.list_uid).await?;
    if task_list.author_uid != user.id()?.parse().unwrap() {
        return Err(Error::new("unauthorized", ErrorType::Unauthorized));
    }
    task_list_service
        .update_task_position(params.list_uid, params.task_uid, payload.position)
        .await?;

    Ok(HttpResponse::Ok().finish())
}
