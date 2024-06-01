use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::prelude::*;
use crate::core::models::task::{CreateTask, Task, UpdateTask};
use crate::core::repositories::task::TaskRepository;
use crate::core::services::task::TaskService;

pub struct TaskServiceImpl {
    pub repository: Arc<dyn TaskRepository>
}

impl TaskServiceImpl {
    pub fn new(repository: Arc<dyn TaskRepository>) -> Self {
        TaskServiceImpl {
            repository
        }
    }
}

#[async_trait]
impl TaskService for TaskServiceImpl {
    async fn create(&self, task: CreateTask) -> Result<Task> {
        self.repository.create(&task).await
    }

    async fn list(&self, author_uid: Uuid) -> Result<Vec<Task>> {
        self.repository.list(author_uid).await
    }

    async fn get(&self, task_id: Uuid) -> Result<Task> {
        self.repository.get(task_id).await
    }

    async fn update(&self, task_id: Uuid, task: UpdateTask) -> Result<Task> {
        self.repository.update(task_id, &task).await
    }

    async fn delete(&self, task_id: Uuid) -> Result<()> {
        self.repository.delete(task_id).await
    }
}
