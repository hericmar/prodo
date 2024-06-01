use std::sync::Arc;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;
use crate::prelude::*;
use crate::core::models::task::{CreateTask, Task, UpdateTask};
use crate::core::repositories::task::TaskRepository;
use crate::infrastructure::databases::postgres::DBPool;

pub struct TaskRepositoryImpl {
    pub pool: Arc<DBPool>
}

impl TaskRepositoryImpl {
    pub fn new(pool: Arc<DBPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn create(&self, new_task: &CreateTask) -> Result<Task> {
        let mut conn = self.pool.get()?;
        diesel::insert_into(crate::schema::tasks::table)
            .values(new_task)
            .get_result(&mut conn)
            .map_err(|e| e.into())
    }

    async fn list(&self, author_uid: Uuid) -> Result<Vec<Task>> {
        let mut conn = self.pool.get()?;
        crate::schema::tasks::table
            .filter(crate::schema::tasks::author_uid.eq(author_uid))
            .load(&mut conn)
            .map_err(|e| e.into())
    }

    async fn get(&self, task_uid: Uuid) -> Result<Task> {
        let mut conn = self.pool.get()?;
        crate::schema::tasks::table
            .filter(crate::schema::tasks::uid.eq(task_uid))
            .first(&mut conn)
            .map_err(|e| e.into())
    }

    async fn update(&self, task_uid: Uuid, task: &UpdateTask) -> Result<Task> {
        let mut conn = self.pool.get()?;
        diesel::update(crate::schema::tasks::table)
            .filter(crate::schema::tasks::uid.eq(task_uid))
            .set(task)
            .get_result(&mut conn)
            .map_err(|e| e.into())
    }

    async fn delete(&self, task_uid: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;
        diesel::delete(crate::schema::tasks::table)
            .filter(crate::schema::tasks::uid.eq(task_uid))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(|e| e.into())
    }
}
