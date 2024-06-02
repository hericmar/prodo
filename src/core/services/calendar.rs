use crate::core::models::calendar::{CalendarSubscription, CreateCalendarSubscription};
use crate::prelude::*;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CalendarService: Send + Sync {
    async fn create_subscription(&self, person_uid: Uuid) -> Result<CalendarSubscription>;
    async fn get_person_subscription(&self, person_uid: Uuid) -> Result<CalendarSubscription>;
    async fn get_subscription(&self, secret: String) -> Result<CalendarSubscription>;

    async fn delete_subscription(&self, person_uid: Uuid) -> Result<()>;
}
