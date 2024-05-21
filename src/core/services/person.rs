use async_trait::async_trait;
use crate::prelude::*;
use crate::core::models::person::{CreatePerson, Person};

#[async_trait]
pub trait PersonService: Sync + Send {
    async fn create(&self, person: CreatePerson) -> Result<Person>;
    async fn list(&self) -> Result<Vec<Person>>;
    async fn get(&self, person_id: i32) -> Result<Person>;
    async fn get_by_username(&self, username: &str) -> Result<Person>;
    async fn delete(&self, person_id: i32) -> Result<usize>;
}
