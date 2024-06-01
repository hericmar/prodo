use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct Task {
    pub uid: Uuid,
    pub summary: String,
    pub description: String,
    pub completed: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
    pub author_uid: Uuid,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct CreateTask {
    pub summary: String,
    pub author_uid: Uuid,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::tasks)]
pub struct UpdateTask {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub completed: Option<DateTime<Utc>>,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::lists)]
pub struct TaskList {
    pub uid: Uuid,
    pub name: String,
    pub author_uid: Uuid,
    pub tasks: Vec<Option<Uuid>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::lists)]
pub struct CreateTaskList {
    pub uid: Option<Uuid>,
    pub name: String,
    pub author_uid: Uuid,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::lists)]
pub struct UpdateTaskList {
    pub name: Option<String>,
    pub tasks: Option<Vec<Option<Uuid>>>,
}
