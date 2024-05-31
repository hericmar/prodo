use async_trait::async_trait;
use uuid::Uuid;
use crate::prelude::*;
use crate::core::models::task::{CreateTask, Task};

#[async_trait]
pub trait TaskService: Sync + Send {
    async fn create(&self, todo: CreateTask) -> Result<Task>;
    async fn list(&self, /*params: TodoQueryParams*/) -> Result<Vec<Task>>;
    async fn get(&self, todo_id: Uuid) -> Result<Task>;
    async fn delete(&self, todo_id: Uuid) -> Result<()>;
}
