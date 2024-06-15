use crate::core::models::task::{
    CreateTask, CreateTaskList, Task, TaskList, UpdateTask, UpdateTaskList, URGENCY_HIGH,
};
use crate::prelude::*;
use crate::schema::tasks::completed;
use async_trait::async_trait;
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use rrule::{RRule, RRuleSet, Unvalidated};
use uuid::Uuid;

#[async_trait]
pub trait TaskService: Sync + Send {
    async fn create(&self, task: CreateTask, task_list_uid: Option<Uuid>) -> Result<Task>;
    async fn list(&self, author_uid: Uuid /*params: TodoQueryParams*/) -> Result<Vec<Task>>;
    async fn get(&self, task_id: Uuid) -> Result<Task>;
    async fn update(&self, task_id: Uuid, task: UpdateTask) -> Result<Task>;
    async fn delete(&self, task_id: Uuid) -> Result<()>;
}

//

pub struct TaskTimeParams {
    pub priority: i32,
    pub dtstart: Option<DateTime<Utc>>,
    pub due: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
    /// Valid recurrence rule string.
    pub rrule: Option<String>,
    pub completed: Option<DateTime<Utc>>,
}

/// Calculate the urgency of a task based on its time parameters.
/// None = 0,
/// Low = 1,
/// Medium = 2,
/// High = 3,
pub fn calculate_urgency(params: TaskTimeParams, now: DateTime<Utc>) -> Option<i32> {
    let since_created = now.signed_duration_since(params.created);

    const BASE_STEP: i32 = 24; // 1 day
    const NUM_SEGMENTS: f32 = 3.0;

    if let Some(rrule) = &params.rrule {
    } else {
        // no recurrence rule
    }

    if params.completed.is_some() && params.rrule.is_none() {
        return None;
    }

    let mut urgency = None;
    let mut percent_elapsed = 0.0;

    if params.rrule.is_some() && params.dtstart.is_some() {
        let rrule_set = into_rrule_set(params.created, &params.rrule.unwrap())?;
        let start = params.completed.unwrap_or(params.created);
        let recurrences = rrule_set
            .after(into_rrule_datetime(start))
            .all(URGENCY_HIGH)
            .dates;
        let missed_occurrences = recurrences.iter().filter(|&r| r < &now).count();
        percent_elapsed = missed_occurrences as f32 / recurrences.len() as f32;
    } else if let Some(due) = params.due {
        let mut since_due = due.signed_duration_since(now).num_hours();

        if since_due < 0 {
            percent_elapsed = 1.0;
        } else {
            let interval_max_size = due.signed_duration_since(params.created).num_hours().abs();
            let since_created = now.signed_duration_since(params.created).num_hours();
            percent_elapsed = since_created as f32 / interval_max_size as f32;
        }
    } else {
        let interval_max_size = BASE_STEP * params.priority;
        percent_elapsed = since_created.num_hours() as f32 / interval_max_size as f32;
    }
    urgency = Some((percent_elapsed * NUM_SEGMENTS).floor() as i32);

    return urgency;
}

//

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

fn into_rrule_datetime(value: DateTime<Utc>) -> DateTime<rrule::Tz> {
    rrule::Tz::UTC
        .with_ymd_and_hms(
            value.year(),
            value.month(),
            value.day(),
            value.hour(),
            value.minute(),
            value.second(),
        )
        .unwrap()
}

fn into_rrule_set(dtstart: DateTime<Utc>, rrule: &String) -> Option<RRuleSet> {
    let rrule_unvalidated: RRule<Unvalidated> = rrule.parse().ok()?;
    let rrule_dtstart = into_rrule_datetime(dtstart);
    let rrule = rrule_unvalidated.validate(rrule_dtstart).ok()?;

    return Some(RRuleSet::new(rrule_dtstart).rrule(rrule));
}
