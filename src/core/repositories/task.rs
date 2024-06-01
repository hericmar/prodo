use async_trait::async_trait;
use uuid::Uuid;
use crate::prelude::*;
use crate::core::models::task::{CreateTask, Task, UpdateTask};

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create(&self, new_task: &CreateTask) -> Result<Task>;
    async fn list(&self, author_uid: Uuid /*params: TodoQueryParams*/) -> Result<Vec<Task>>;
    async fn get(&self, task_uid: Uuid) -> Result<Task>;
    async fn update(&self, task_uid: Uuid, task: &UpdateTask) -> Result<Task>;
    async fn delete(&self, task_uid: Uuid) -> Result<()>;
}
