use std::sync::Arc;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::result::DatabaseErrorKind;
use uuid::Uuid;
use crate::prelude::*;
use crate::core::models::person::{CreatePerson, Person};
use crate::core::repositories::person::PersonRepository;
use crate::error::{Error, ErrorType};
use crate::infrastructure::databases::postgres::DBPool;

pub struct PersonRepositoryImpl {
    pub pool: Arc<DBPool>
}

impl PersonRepositoryImpl {
    pub fn new(pool: Arc<DBPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PersonRepository for PersonRepositoryImpl {
    async fn create(&self, person: &CreatePerson) -> Result<Person> {
        let mut conn = self.pool.get()?;
        diesel::insert_into(crate::schema::persons::table)
            .values(person)
            .returning(Person::as_returning())
            .get_result(&mut conn)
            .map_err(|e| e.into())
    }

    async fn list(&self) -> Result<Vec<Person>> {
        let mut conn = self.pool.get()?;
        crate::schema::persons::table
            .select(Person::as_select())
            .load(&mut conn)
            .map_err(|e| e.into())
    }

    async fn get(&self, person_uid: Uuid) -> Result<Person> {
        todo!()
    }

    async fn get_by_username(&self, username: &str) -> Result<Person> {
        let mut conn = self.pool.get()?;
        crate::schema::persons::table
            .filter(crate::schema::persons::username.eq(username))
            .first(&mut conn)
            .map_err(|e| e.into())
    }

    async fn delete(&self, person_uid: Uuid) -> Result<usize> {
        let mut conn = self.pool.get()?;
        diesel::delete(crate::schema::persons::table)
            .filter(crate::schema::persons::uid.eq(person_uid))
            .execute(&mut conn)
            .map_err(|e| e.into())
    }
}
