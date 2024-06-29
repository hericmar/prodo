use crate::core::models::task::{CreateTask, CreateTaskList, UpdateTask, UpdateTaskList};
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

#[derive(Deserialize)]
pub struct DeleteTaskPathParams {
    pub list_uid: Uuid,
    pub task_uid: Uuid,
}

pub async fn delete_task_handler(
    path: web::Path<DeleteTaskPathParams>,
    user: Identity,
    task_service: web::Data<dyn TaskService>,
) -> Result<HttpResponse> {
    let task_uid = path.task_uid;
    let task = task_service.get(task_uid.clone()).await?;
    if task.author_uid != user.id()?.parse().unwrap() {
        return Err(Error::new("unauthorized", ErrorType::Unauthorized));
    }

    task_service.delete(path.list_uid, task_uid).await?;

    Ok(HttpResponse::Ok().finish())
}

//

#[derive(Deserialize)]
pub struct CreateUpdateTaskListRequest {
    pub name: String,
}

pub async fn create_task_list_handler(
    user: Identity,
    task_list_service: web::Data<dyn TaskListService>,
    payload: web::Json<CreateUpdateTaskListRequest>,
) -> Result<HttpResponse> {
    let new_task_list = CreateTaskList {
        uid: None,
        name: payload.name.clone(),
        author_uid: user.id()?.parse().unwrap(),
    };
    let result = task_list_service.create(new_task_list).await?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_task_list_handler(
    list_uid: web::Path<Uuid>,
    user: Identity,
    task_list_service: web::Data<dyn TaskListService>,
    payload: web::Json<CreateUpdateTaskListRequest>,
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
pub struct DeleteTaskListParams {
    // TODO: Not yet supported, will be implemented in the future.
    pub delete_orphans: Option<bool>,
    pub target_list: Option<Uuid>,
}

pub async fn delete_task_list_handler(
    list_uid: web::Path<Uuid>,
    user: Identity,
    params: web::Query<DeleteTaskListParams>,
    task_service: web::Data<dyn TaskService>,
    task_list_service: web::Data<dyn TaskListService>,
) -> Result<HttpResponse> {
    let list_uid = list_uid.into_inner();
    let task_list = task_list_service.get(list_uid.clone()).await?;
    if task_list.author_uid != user.id()?.parse().unwrap() {
        return Err(Error::new("unauthorized", ErrorType::Unauthorized));
    }

    if let Some(delete_orphans) = params.delete_orphans {
        if delete_orphans {
            for task_uid in &task_list.tasks {
                let _ = task_service.delete(list_uid, task_uid.unwrap()).await;
            }
        }
    }
    if params.target_list.is_some() {
        task_list_service
            .move_tasks(list_uid, params.target_list, task_list.tasks)
            .await?;
    }

    task_list_service.delete(list_uid).await?;

    Ok(HttpResponse::Ok().finish())
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

#[derive(Deserialize)]
pub struct MoveTaskRequest {
    pub target_list_uid: Uuid,
}

#[derive(Deserialize)]
pub struct MoveTaskPathParams {
    pub list_uid: Uuid,
    pub task_uid: Uuid,
}

pub async fn move_task_handler(
    path: web::Path<MoveTaskPathParams>,
    user: Identity,
    task_service: web::Data<dyn TaskService>,
    task_list_service: web::Data<dyn TaskListService>,
    payload: web::Json<MoveTaskRequest>,
) -> Result<HttpResponse> {
    let list_uid = path.list_uid;
    let task_uid = path.task_uid;
    let task = task_service.get(task_uid.clone()).await?;
    if task.author_uid != user.id()?.parse().unwrap() {
        return Err(Error::new("unauthorized", ErrorType::Unauthorized));
    }

    task_list_service
        .move_tasks(
            list_uid,
            Some(payload.target_list_uid),
            vec![Some(task_uid)],
        )
        .await?;

    Ok(HttpResponse::Ok().finish())
}
