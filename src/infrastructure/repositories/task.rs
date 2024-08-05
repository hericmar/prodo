use crate::core::models::task::{
    CreateTask, CreateTaskList, Task, TaskList, UpdateTask, UpdateTaskList,
};
use crate::core::repositories::task::{ListTaskParams, TaskListRepository, TaskRepository};
use crate::infrastructure::databases::postgres::DBPool;
use crate::prelude::*;
use async_trait::async_trait;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use std::collections::HashSet;
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

    async fn list(&self, author_uid: Uuid, params: ListTaskParams) -> Result<Vec<Task>> {
        let mut conn = self.pool.get()?;

        let mut query = crate::schema::tasks::table
            .into_boxed()
            .filter(crate::schema::tasks::author_uid.eq(author_uid));

        if !params.include_archived {
            query = query.filter(crate::schema::tasks::is_archived.eq(false));
        }

        query.load(&mut conn).map_err(|e| e.into())
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

    async fn archive(&self, task_uid: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;
        diesel::update(crate::schema::tasks::table)
            .filter(crate::schema::tasks::uid.eq(task_uid))
            .set(crate::schema::tasks::is_archived.eq(true))
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

    async fn archive_tasks(&self, list_uid: Uuid, tasks_to_archive: Vec<Uuid>) -> Result<()> {
        let mut conn = self.pool.get()?;
        conn.transaction(|conn| {
            let list: TaskList = crate::schema::lists::table
                .filter(crate::schema::lists::uid.eq(list_uid))
                .first(conn)?;

            diesel::update(crate::schema::tasks::table)
                .filter(crate::schema::tasks::uid.eq_any(&tasks_to_archive))
                .set(crate::schema::tasks::is_archived.eq(true))
                .execute(conn)?;

            // remove tasks to be archived from list
            let mut updated_tasks = list.tasks.clone();
            updated_tasks.retain(|t| !tasks_to_archive.contains(&t.unwrap()));

            diesel::update(crate::schema::lists::table)
                .filter(crate::schema::lists::uid.eq(list_uid))
                .set(crate::schema::lists::tasks.eq(updated_tasks))
                .execute(conn)?;

            Ok(())
        })
    }

    async fn delete(&self, list_uid: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;
        diesel::delete(crate::schema::lists::table)
            .filter(crate::schema::lists::uid.eq(list_uid))
            .execute(&mut conn)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    async fn move_tasks(
        &self,
        source_list_uid: Uuid,
        target_list_uid: Option<Uuid>,
        tasks: Vec<Option<Uuid>>,
    ) -> Result<()> {
        let source_list = self.get(source_list_uid).await?;

        let tasks_set = tasks
            .iter()
            .cloned()
            .map(|uid| uid.unwrap_or(Uuid::default()))
            .collect::<HashSet<Uuid>>();
        let source_tasks_set = source_list
            .tasks
            .iter()
            .map(|uid| uid.unwrap_or(Uuid::default()))
            .collect::<HashSet<Uuid>>();

        let target_tasks_set = match target_list_uid {
            None => HashSet::new(),
            Some(target_list_uid) => {
                let target_list = self.get(target_list_uid).await?;

                target_list
                    .tasks
                    .iter()
                    .map(|uid| uid.unwrap_or(Uuid::default()))
                    .collect::<HashSet<Uuid>>()
            }
        };

        let source_tasks_without_removed = source_tasks_set
            .difference(&tasks_set)
            .map(|uid| Some(*uid))
            .collect::<Vec<Option<Uuid>>>();
        // Not needed for now, task cannot be in multiple lists.
        // let to_add = tasks.iter().filter(|t| !target_tasks_set.contains(t)).collect::<Vec<_>>();

        self.update(
            source_list_uid,
            &UpdateTaskList {
                name: None,
                is_archived: None,
                tasks: Some(source_tasks_without_removed),
            },
        )
        .await?;

        if let Some(target_list_uid) = target_list_uid {
            let target_tasks_with_added = target_tasks_set
                .union(&tasks_set)
                .map(|uid| Some(*uid))
                .collect::<Vec<Option<Uuid>>>();

            self.update(
                target_list_uid,
                &UpdateTaskList {
                    name: None,
                    is_archived: None,
                    tasks: Some(target_tasks_with_added),
                },
            )
            .await?;
        }

        Ok(())
    }
}
