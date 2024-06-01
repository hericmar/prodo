use crate::core::models::person::{CreatePerson, Person};
use crate::core::models::task::CreateTaskList;
use crate::core::repositories::person::PersonRepository;
use crate::core::repositories::task::TaskListRepository;
use crate::core::services::person::PersonService;
use crate::error::{Error, ErrorType};
use crate::prelude::*;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct PersonServiceImpl {
    pub repository: Arc<dyn PersonRepository>,
    pub task_list_repository: Arc<dyn TaskListRepository>,
}

impl PersonServiceImpl {
    pub fn new(
        repository: Arc<dyn PersonRepository>,
        task_list_repository: Arc<dyn TaskListRepository>,
    ) -> Self {
        PersonServiceImpl {
            repository,
            task_list_repository,
        }
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Error::new(&err.to_string(), ErrorType::InternalError)
    }
}

#[async_trait]
impl PersonService for PersonServiceImpl {
    async fn create(&self, mut person: CreatePerson) -> Result<Person> {
        if person.password.is_empty() {
            return Err(Error::new(
                "Password cannot be empty",
                ErrorType::BadRequest,
            ));
        }

        let salt = SaltString::generate(&mut OsRng);
        let hasher = Argon2::default();
        person.password = hasher
            .hash_password(person.password.as_ref(), &salt)?
            .to_string();
        person.uid = Some(Uuid::new_v4());

        let result = self.repository.create(&person).await;

        self.task_list_repository
            .create(&CreateTaskList {
                uid: Some(Uuid::new_v4()),
                name: "Tasks".to_string(),
                author_uid: person.uid.unwrap(),
            })
            .await?;

        return result;
    }

    async fn list(&self) -> Result<Vec<Person>> {
        self.repository.list().await
    }

    async fn get(&self, person_uid: Uuid) -> Result<Person> {
        self.repository.get(person_uid).await
    }

    async fn get_by_username(&self, username: &str) -> Result<Person> {
        self.repository.get_by_username(username).await
    }

    async fn delete(&self, person_uid: Uuid) -> Result<usize> {
        self.repository.delete(person_uid).await
    }
}
