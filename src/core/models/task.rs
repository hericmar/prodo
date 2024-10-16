use crate::core::services::task::{into_rrule_set, TaskTimeParams};
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rrule::Tz;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub const URGENCY_NONE: u16 = 0;
pub const URGENCY_LOW: u16 = 1;
pub const URGENCY_MEDIUM: u16 = 2;
pub const URGENCY_HIGH: u16 = 3;

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
    pub dtend: Option<DateTime<Utc>>,
    /// 1 is the highest priority and 9 is the lowest, 0 is unset and 5 is medium.
    pub priority: i32,
    pub due: Option<DateTime<Utc>>,
    pub sequence: i32,
    pub urgency: i32,
    #[serde(skip)]
    pub is_archived: bool,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct CreateTask {
    pub summary: String,
    pub author_uid: Uuid,
}

#[derive(Default, Deserialize, AsChangeset, Validate)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(treat_none_as_null = true)]
pub struct UpdateTask {
    pub summary: String,
    pub description: String,
    pub completed: Option<DateTime<Utc>>,
    pub rrule: Option<String>,
    pub dtstart: Option<DateTime<Utc>>,
    pub dtend: Option<DateTime<Utc>>,
    #[validate(range(min = 0, max = 9))]
    pub priority: i32,
    // #[validate(range(min = 0, max = 100))]
    // pub percent_complete: i32,
    pub due: Option<DateTime<Utc>>,

    #[serde(skip)]
    pub sequence: i32,
    #[serde(skip)]
    pub urgency: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::tasks)]
pub struct ArchiveTask {
    pub is_archived: bool,
}

impl Task {
    pub fn time_params(&self, created: DateTime<Utc>) -> TaskTimeParams {
        TaskTimeParams {
            priority: self.priority,
            dtstart: self.dtstart,
            due: self.due,
            created,
            rrule: self.rrule.clone(),
            completed: self.completed,
        }
    }

    pub fn has_next_occurrence(&self, now: DateTime<Tz>) -> bool {
        if let Some(rrule) = &self.rrule {
            let rrule_set = into_rrule_set(self.created, rrule).unwrap();
            let next_occurrence = rrule_set.after(now).all(5).dates;

            next_occurrence.is_empty()
        } else {
            false
        }
    }
}

impl UpdateTask {
    pub fn time_params(&self, created: DateTime<Utc>) -> TaskTimeParams {
        TaskTimeParams {
            priority: self.priority,
            dtstart: self.dtstart,
            due: self.due,
            created,
            rrule: self.rrule.clone(),
            completed: self.completed,
        }
    }
}

//

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::lists)]
pub struct TaskList {
    pub uid: Uuid,
    pub name: String,
    pub author_uid: Uuid,
    pub tasks: Vec<Option<Uuid>>,
    pub is_archived: bool,
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
    pub is_archived: Option<bool>,
}
