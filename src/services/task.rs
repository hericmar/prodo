use crate::core::models::task::{
    CreateTask, CreateTaskList, Task, TaskList, UpdateTask, UpdateTaskList,
};
use crate::core::repositories::task::{ListTaskParams, TaskListRepository, TaskRepository};
use crate::core::services::person::PersonService;
use crate::core::services::task::{
    calculate_urgency, into_rrule_datetime, into_rrule_set, TaskListService, TaskService,
};
use crate::error::{Error, ErrorType};
use crate::infrastructure::cron::CronJob;
use crate::prelude::*;
use async_trait::async_trait;
use chrono::Utc;
use rrule::{RRule, RRuleError, RRuleSet, Unvalidated};
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
    async fn create(&self, new_task: CreateTask, task_list_uid: Uuid) -> Result<Task> {
        let task = self.repository.create(&new_task).await?;
        let mut list = self.task_list_repository.get(task_list_uid).await?;

        list.tasks.insert(0, Some(task.uid));
        self.task_list_repository
            .update(
                list.uid,
                &UpdateTaskList {
                    name: None,
                    is_archived: None,
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

        let mut tasks = self
            .repository
            .list(author_uid, ListTaskParams::default())
            .await?;
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

    async fn update(&self, task_id: Uuid, mut task: UpdateTask) -> Result<Task> {
        task.validate()?;

        // validate priority
        if task.priority < 0 || task.priority > 9 {
            return Err(Error::new("Invalid priority", ErrorType::BadRequest));
        }

        let old_task = self.repository.get(task_id).await?;

        // validate dtstart and dtend
        if let (Some(dtstart), Some(dtend)) = (task.dtstart, task.dtend) {
            if dtstart > dtend {
                return Err(Error::new(
                    "Invalid time range for `dtstart` and `dtend`",
                    ErrorType::BadRequest,
                ));
            }
        }

        if let (Some(dtstart), Some(due)) = (task.dtstart, task.due) {
            if dtstart > due {
                return Err(Error::new(
                    "Invalid time range for `dtstart` and `due`",
                    ErrorType::BadRequest,
                ));
            }
        }

        // calculate urgency
        let params = task.time_params(old_task.created);
        if let Some(urgency) = calculate_urgency(params, Utc::now()) {
            task.urgency = urgency;
        }

        task.sequence = old_task.sequence + 1;

        self.repository.update(task_id, &task).await
    }

    async fn update_urgency(&self, task_id: Uuid, urgency: i32) -> Result<()> {
        self.repository.update_urgency(task_id, urgency).await
    }

    async fn delete(&self, list_uid: Uuid, task_uid: Uuid) -> Result<()> {
        // TODO: use transaction
        self.repository.delete(task_uid).await?;
        Ok(self
            .task_list_repository
            .move_tasks(list_uid, None, vec![Some(task_uid)])
            .await?)
    }

    async fn archive(&self, task_id: Uuid) -> Result<()> {
        self.repository.archive(task_id).await
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
                    is_archived: None,
                    tasks: Some(list.tasks),
                },
            )
            .await
    }

    async fn move_tasks(
        &self,
        source_list_uid: Uuid,
        target_list_uid: Option<Uuid>,
        tasks: Vec<Option<Uuid>>,
    ) -> Result<()> {
        self.repository
            .move_tasks(source_list_uid, target_list_uid, tasks)
            .await
    }

    async fn archive_tasks(&self, list_uid: Uuid, tasks: Vec<Uuid>) -> Result<()> {
        self.repository.archive_tasks(list_uid, tasks).await
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

//

pub struct UpdateTaskUrgencyJob {
    pub person_service: Arc<dyn PersonService>,
    pub task_service: Arc<dyn TaskService>,
}

impl UpdateTaskUrgencyJob {
    pub fn new(person_service: Arc<dyn PersonService>, task_service: Arc<dyn TaskService>) -> Self {
        UpdateTaskUrgencyJob {
            person_service,
            task_service,
        }
    }
}

#[async_trait]
impl CronJob for UpdateTaskUrgencyJob {
    async fn run(&self) -> Result<()> {
        let people = self.person_service.list().await?;
        for person in people {
            let tasks = self.task_service.list(person.uid).await?;
            for task in &tasks {
                let params = task.time_params(task.created);
                if let Some(urgency) = calculate_urgency(params, Utc::now()) {
                    self.task_service.update_urgency(task.uid, urgency).await?;
                }
            }
        }

        Ok(())
    }
}

//

pub struct ArchiveCompletedTasksJob {
    pub person_service: Arc<dyn PersonService>,
    pub task_service: Arc<dyn TaskService>,
    pub task_list_service: Arc<dyn TaskListService>,
}

impl ArchiveCompletedTasksJob {
    pub fn new(
        person_service: Arc<dyn PersonService>,
        task_service: Arc<dyn TaskService>,
        task_list_service: Arc<dyn TaskListService>,
    ) -> Self {
        ArchiveCompletedTasksJob {
            person_service,
            task_service,
            task_list_service,
        }
    }
}

#[async_trait]
impl CronJob for ArchiveCompletedTasksJob {
    async fn run(&self) -> Result<()> {
        let now = into_rrule_datetime(Utc::now());
        let people = self.person_service.list().await?;

        for person in people {
            let tasks = self.task_service.list(person.uid).await?;
            let lists = self.task_list_service.list(person.uid).await?;

            let mut tasks_to_archive = vec![];

            for task in &tasks {
                let completed_before_60_days = task.completed.map_or(false, |completed| {
                    completed < now - chrono::Duration::days(60)
                });

                if completed_before_60_days && !task.has_next_occurrence(now) {
                    tasks_to_archive.push(task.uid);
                }
            }

            for list in &lists {
                self.task_list_service
                    .archive_tasks(list.uid, tasks_to_archive.clone())
                    .await?;
            }
        }

        Ok(())
    }
}

//

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
