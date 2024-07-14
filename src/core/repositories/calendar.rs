use crate::core::models::calendar::{
    CalendarSubscription, CreateCalendarSubscription, UpdateCalendarSubscription,
};
use crate::prelude::*;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CalendarSubscriptionRepository: Send + Sync {
    async fn create(
        &self,
        calendar_subscription: CreateCalendarSubscription,
    ) -> Result<CalendarSubscription>;
    async fn get(&self, secret: &String) -> Result<CalendarSubscription>;
    async fn get_person_subscription(&self, person_uid: Uuid) -> Result<CalendarSubscription>;
    async fn update(
        &self,
        person_uid: Uuid,
        payload: &UpdateCalendarSubscription,
    ) -> Result<CalendarSubscription>;
    async fn delete(&self, person_uid: Uuid) -> Result<()>;
}
