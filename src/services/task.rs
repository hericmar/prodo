use crate::core::models::task::{
    CreateTask, CreateTaskList, Task, TaskList, UpdateTask, UpdateTaskList,
};
use crate::core::repositories::task::{TaskListRepository, TaskRepository};
use crate::core::services::task::{TaskListService, TaskService};
use crate::error::{Error, ErrorType};
use crate::prelude::*;
use async_trait::async_trait;
use chrono::{Datelike, TimeZone, Timelike, Utc};
use rrule::{RRule, RRuleError, RRuleSet, Tz, Unvalidated};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct TaskServiceImpl {
    pub repository: Arc<dyn TaskRepository>,
    pub task_list_repository: Arc<dyn TaskListRepository>,
}

impl TaskServiceImpl {
    pub fn new(
        repository: Arc<dyn TaskRepository>,
        task_list_repository: Arc<dyn TaskListRepository>,
    ) -> Self {
        TaskServiceImpl {
            repository,
            task_list_repository,
        }
    }
}

#[async_trait]
impl TaskService for TaskServiceImpl {
    async fn create(&self, new_task: CreateTask, task_list_uid: Option<Uuid>) -> Result<Task> {
        let task = self.repository.create(&new_task).await?;
        let mut lists = self.task_list_repository.list(new_task.author_uid).await?;
        // TODO: Only one list for now
        if lists.is_empty() {
            return Err(Error::new("List not found", ErrorType::NotFound));
        }
        let mut list = &mut lists[0];
        list.tasks.insert(0, Some(task.uid));
        self.task_list_repository
            .update(
                list.uid,
                &UpdateTaskList {
                    name: None,
                    tasks: Some(list.tasks.clone()),
                },
            )
            .await?;

        Ok(task)
    }

    async fn list(&self, author_uid: Uuid) -> Result<Vec<Task>> {
        let lists = self.task_list_repository.list(author_uid).await?;
        if lists.is_empty() {
            return Ok(vec![]);
        }
        let list = &lists[0];

        let mut tasks = self.repository.list(author_uid).await?;
        tasks.sort_by(|a, b| {
            let a_pos = list.tasks.iter().position(|t_uid| t_uid == &Some(a.uid));
            let b_pos = list.tasks.iter().position(|t_uid| t_uid == &Some(b.uid));
            a_pos.cmp(&b_pos)
        });

        Ok(tasks)
    }

    async fn get(&self, task_id: Uuid) -> Result<Task> {
        self.repository.get(task_id).await
    }

    async fn update(&self, task_id: Uuid, task: UpdateTask) -> Result<Task> {
        task.validate()?;

        let old_task = self.repository.get(task_id).await?;

        if let Some(rrule) = &task.rrule {
            let rrule_unvalidated: RRule<Unvalidated> = rrule.parse()?;

            let dtstart = task.dtstart.or(old_task.dtstart);
            if let Some(dtstart) = dtstart {
                let rrule_dtstart = Tz::UTC
                    .with_ymd_and_hms(
                        dtstart.year(),
                        dtstart.month(),
                        dtstart.day(),
                        dtstart.hour(),
                        dtstart.minute(),
                        dtstart.second(),
                    )
                    .unwrap();
                let rrule = rrule_unvalidated.validate(rrule_dtstart)?;
            }
        }

        self.repository.update(task_id, &task).await
    }

    async fn delete(&self, task_id: Uuid) -> Result<()> {
        self.repository.delete(task_id).await
    }
}

//

pub struct TaskListServiceImpl {
    pub repository: Arc<dyn TaskListRepository>,
}

impl TaskListServiceImpl {
    pub fn new(repository: Arc<dyn TaskListRepository>) -> Self {
        TaskListServiceImpl { repository }
    }
}

#[async_trait]
impl TaskListService for TaskListServiceImpl {
    async fn create(&self, task: CreateTaskList) -> Result<TaskList> {
        self.repository.create(&task).await
    }

    async fn get(&self, task_id: Uuid) -> Result<TaskList> {
        self.repository.get(task_id).await
    }

    async fn list(&self, author_uid: Uuid) -> Result<Vec<TaskList>> {
        self.repository.list(author_uid).await
    }

    async fn update(&self, task_id: Uuid, task: UpdateTaskList) -> Result<TaskList> {
        self.repository.update(task_id, &task).await
    }

    async fn update_task_position(
        &self,
        list_uid: Uuid,
        task_uid: Uuid,
        position: i32,
    ) -> Result<TaskList> {
        let mut list = self.repository.get(list_uid).await?;
        if position < 0 || position as usize >= list.tasks.len() {
            return Err(Error::new("Invalid position", ErrorType::BadRequest));
        }

        let maybe_old_position = list.tasks.iter().position(|t_uid| t_uid == &Some(task_uid));
        if maybe_old_position.is_none() {
            return Err(Error::new("Task not found in list", ErrorType::NotFound));
        }

        let old_position = maybe_old_position.unwrap();
        let task_uid = list.tasks.remove(old_position).unwrap();
        list.tasks.insert(position as usize, Some(task_uid));

        self.repository
            .update(
                list_uid,
                &UpdateTaskList {
                    name: None,
                    tasks: Some(list.tasks),
                },
            )
            .await
    }

    async fn delete(&self, task_id: Uuid) -> Result<()> {
        self.repository.delete(task_id).await
    }
}

impl From<RRuleError> for Error {
    fn from(value: RRuleError) -> Self {
        Error::new(&value.to_string(), ErrorType::BadRequest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rrule::{Frequency, RRule, RRuleSet, Tz, Unvalidated};

    #[test]
    fn test_rrule() {
        let rrule: RRule<Unvalidated> = "FREQ=DAILY;COUNT=40;INTERVAL=3".parse().unwrap();
        assert_eq!(rrule.get_freq(), Frequency::Daily);
        assert_eq!(rrule.get_count(), Some(40));
        assert_eq!(rrule.get_interval(), 3);
    }
}
