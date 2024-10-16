use crate::core::models::calendar::{CalendarSubscription, UpdateCalendarSubscription};
use crate::prelude::*;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CalendarService: Send + Sync {
    async fn create_subscription(
        &self,
        person_uid: Uuid,
        timezone: &str,
    ) -> Result<CalendarSubscription>;
    async fn get_person_subscription(&self, person_uid: Uuid) -> Result<CalendarSubscription>;
    async fn get_subscription(&self, secret: String) -> Result<CalendarSubscription>;
    async fn update_subscription(
        &self,
        person_uid: Uuid,
        payload: &UpdateCalendarSubscription,
    ) -> Result<CalendarSubscription>;
    async fn update_last_synced_at(
        &self,
        person_uid: Uuid,
        last_synced_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<CalendarSubscription>;
    async fn delete_subscription(&self, person_uid: Uuid) -> Result<()>;
}
