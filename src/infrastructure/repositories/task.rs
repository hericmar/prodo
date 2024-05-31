use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::prelude::*;
use crate::core::models::task::{CreateTask, Task};
use crate::core::repositories::task::TaskRepository;
use crate::infrastructure::databases::postgres::DBPool;

struct TaskRepositoryImpl {
    pub pool: Arc<DBPool>
}

impl TaskRepositoryImpl {
    pub fn new(pool: Arc<DBPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn create(&self, new_todo: &CreateTask) -> Result<Task> {
        let mut conn = self.pool.get()?;
        todo!()
    }

    async fn list(&self) -> Result<Vec<Task>> {
        todo!()
    }

    async fn get(&self, task_uid: Uuid) -> Result<Task> {
        todo!()
    }

    async fn delete(&self, task_uid: Uuid) -> Result<()> {
        todo!()
    }
}
