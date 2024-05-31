use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::prelude::*;
use crate::core::models::task::{CreateTask, Task};
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
    async fn create(&self, todo: CreateTask) -> Result<Task> {
        todo!()
    }

    async fn list(&self) -> Result<Vec<Task>> {
        todo!()
    }

    async fn get(&self, todo_id: Uuid) -> Result<Task> {
        todo!()
    }

    async fn delete(&self, todo_id: Uuid) -> Result<()> {
        todo!()
    }
}
