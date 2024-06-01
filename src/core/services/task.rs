use crate::core::models::task::{
    CreateTask, CreateTaskList, Task, TaskList, UpdateTask, UpdateTaskList,
};
use crate::prelude::*;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TaskService: Sync + Send {
    async fn create(&self, task: CreateTask, task_list_uid: Option<Uuid>) -> Result<Task>;
    async fn list(&self, author_uid: Uuid /*params: TodoQueryParams*/) -> Result<Vec<Task>>;
    async fn get(&self, task_id: Uuid) -> Result<Task>;
    async fn update(&self, task_id: Uuid, task: UpdateTask) -> Result<Task>;
    async fn delete(&self, task_id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait TaskListService: Sync + Send {
    async fn create(&self, task: CreateTaskList) -> Result<TaskList>;
    async fn get(&self, task_id: Uuid) -> Result<TaskList>;
    async fn list(&self, author_uid: Uuid) -> Result<Vec<TaskList>>;
    async fn update(&self, task_id: Uuid, task: UpdateTaskList) -> Result<TaskList>;

    async fn update_task_position(
        &self,
        list_uid: Uuid,
        task_uid: Uuid,
        position: i32,
    ) -> Result<TaskList>;

    async fn delete(&self, task_id: Uuid) -> Result<()>;
}
