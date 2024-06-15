use crate::core::models::person::{CreatePerson, Person};
use crate::prelude::*;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PersonRepository: Send + Sync {
    async fn create(&self, person: &CreatePerson) -> Result<Person>;
    async fn list(&self) -> Result<Vec<Person>>;
    async fn get(&self, person_uid: Uuid) -> Result<Person>;
    async fn get_by_username(&self, username: &str) -> Result<Person>;
    async fn delete(&self, person_uid: Uuid) -> Result<usize>;
}
