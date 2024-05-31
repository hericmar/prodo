use std::sync::Arc;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use async_trait::async_trait;
use uuid::Uuid;
use crate::prelude::*;
use crate::core::models::person::{CreatePerson, Person};
use crate::core::repositories::person::PersonRepository;
use crate::core::services::person::PersonService;
use crate::error::{Error, ErrorType};

pub struct PersonServiceImpl {
    pub repository: Arc<dyn PersonRepository>
}

impl PersonServiceImpl {
    pub fn new(repository: Arc<dyn PersonRepository>) -> Self {
        PersonServiceImpl {
            repository
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
            return Err(Error::new("Password cannot be empty", ErrorType::BadRequest));
        }

        let salt = SaltString::generate(&mut OsRng);
        let hasher = Argon2::default();
        person.password = hasher.hash_password(person.password.as_ref(), &salt)?.to_string();

        self.repository.create(&person).await
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
