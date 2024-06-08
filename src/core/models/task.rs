use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct Task {
    pub uid: Uuid,
    pub summary: String,
    pub description: String,
    pub completed: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
    pub author_uid: Uuid,
    pub rrule: Option<String>,
    pub dtstart: Option<DateTime<Utc>>,
    pub priority: i32,
    pub percent_complete: i32,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct CreateTask {
    pub summary: String,
    pub author_uid: Uuid,
}

#[derive(Deserialize, AsChangeset, Validate)]
#[diesel(table_name = crate::schema::tasks)]
pub struct UpdateTask {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub completed: Option<DateTime<Utc>>,
    pub rrule: Option<String>,
    pub dtstart: Option<DateTime<Utc>>,
    #[validate(range(min = 0, max = 9))]
    pub priority: Option<i32>,
    #[validate(range(min = 0, max = 100))]
    pub percent_complete: Option<i32>,
}

//

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
