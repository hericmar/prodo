use crate::core::models::task::{
    CreateTask, CreateTaskList, Task, TaskList, UpdateTask, UpdateTaskList,
};
use crate::prelude::*;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create(&self, new_task: &CreateTask) -> Result<Task>;
    async fn list(&self, author_uid: Uuid /*params: TodoQueryParams*/) -> Result<Vec<Task>>;
    async fn get(&self, task_uid: Uuid) -> Result<Task>;
    async fn update(&self, task_uid: Uuid, task: &UpdateTask) -> Result<Task>;
    async fn update_urgency(&self, task_uid: Uuid, urgency: i32) -> Result<()>;
    async fn delete(&self, task_uid: Uuid) -> Result<()>;
}

#[async_trait]
pub trait TaskListRepository: Send + Sync {
    async fn create(&self, new_list: &CreateTaskList) -> Result<TaskList>;
    async fn list(&self, author_uid: Uuid) -> Result<Vec<TaskList>>;
    async fn get(&self, list_uid: Uuid) -> Result<TaskList>;
    async fn update(&self, list_uid: Uuid, list: &UpdateTaskList) -> Result<TaskList>;
    async fn delete(&self, list_uid: Uuid) -> Result<()>;
}
