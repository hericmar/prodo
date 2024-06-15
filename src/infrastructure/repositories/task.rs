use crate::core::models::task::{
    CreateTask, CreateTaskList, Task, TaskList, UpdateTask, UpdateTaskList,
};
use crate::core::repositories::task::{TaskListRepository, TaskRepository};
use crate::infrastructure::databases::postgres::DBPool;
use crate::prelude::*;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::sync::Arc;
use uuid::Uuid;

pub struct TaskRepositoryImpl {
    pub pool: Arc<DBPool>,
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

    async fn update_urgency(&self, task_uid: Uuid, urgency: i32) -> Result<()> {
        let mut conn = self.pool.get()?;
        diesel::update(crate::schema::tasks::table)
            .filter(crate::schema::tasks::uid.eq(task_uid))
            .set(crate::schema::tasks::urgency.eq(urgency))
            .execute(&mut conn)
            .map(|_| ())
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

pub struct TaskListRepositoryImpl {
    pub pool: Arc<DBPool>,
}

impl TaskListRepositoryImpl {
    pub fn new(pool: Arc<DBPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskListRepository for TaskListRepositoryImpl {
    async fn create(&self, new_list: &CreateTaskList) -> Result<TaskList> {
        let mut conn = self.pool.get()?;
        diesel::insert_into(crate::schema::lists::table)
            .values(new_list)
            .get_result(&mut conn)
            .map_err(|e| e.into())
    }

    async fn list(&self, author_uid: Uuid) -> Result<Vec<TaskList>> {
        let mut conn = self.pool.get()?;
        crate::schema::lists::table
            .filter(crate::schema::lists::author_uid.eq(author_uid))
            .load(&mut conn)
            .map_err(|e| e.into())
    }

    async fn get(&self, list_uid: Uuid) -> Result<TaskList> {
        let mut conn = self.pool.get()?;
        crate::schema::lists::table
            .filter(crate::schema::lists::uid.eq(list_uid))
            .first(&mut conn)
            .map_err(|e| e.into())
    }

    async fn update(&self, list_uid: Uuid, list: &UpdateTaskList) -> Result<TaskList> {
        let mut conn = self.pool.get()?;
        diesel::update(crate::schema::lists::table)
            .filter(crate::schema::lists::uid.eq(list_uid))
            .set(list)
            .get_result(&mut conn)
            .map_err(|e| e.into())
    }

    async fn delete(&self, list_uid: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;
        diesel::delete(crate::schema::lists::table)
            .filter(crate::schema::lists::uid.eq(list_uid))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(|e| e.into())
    }
}
