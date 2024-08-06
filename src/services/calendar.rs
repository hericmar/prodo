use crate::core::models::calendar::{
    CalendarSubscription, CreateCalendarSubscription, UpdateCalendarSubscription,
};
use crate::core::repositories::calendar::CalendarSubscriptionRepository;
use crate::core::services::calendar::CalendarService;
use crate::error::{Error, ErrorType};
use crate::prelude::*;
use async_trait::async_trait;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct CalendarServiceImpl {
    pub repository: Arc<dyn CalendarSubscriptionRepository>,
}

impl CalendarServiceImpl {
    pub fn new(repository: Arc<dyn CalendarSubscriptionRepository>) -> Self {
        CalendarServiceImpl { repository }
    }
}

#[async_trait]
impl CalendarService for CalendarServiceImpl {
    async fn create_subscription(
        &self,
        person_uid: Uuid,
        timezone: &str,
    ) -> Result<CalendarSubscription> {
        chrono_tz::Tz::from_str(timezone)
            .map_err(|_| Error::new("Invalid timezone", ErrorType::BadRequest))?;

        let secret: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(255)
            .map(char::from)
            .collect();
        let subscription = CreateCalendarSubscription {
            person_uid,
            secret: secret.clone(),
            timezone: timezone.to_string(),
        };

        self.repository.create(subscription).await
    }

    async fn get_person_subscription(&self, person_uid: Uuid) -> Result<CalendarSubscription> {
        self.repository.get_person_subscription(person_uid).await
    }

    async fn get_subscription(&self, secret: String) -> Result<CalendarSubscription> {
        self.repository.get(&secret).await
    }

    async fn update_subscription(
        &self,
        person_uid: Uuid,
        payload: &UpdateCalendarSubscription,
    ) -> Result<CalendarSubscription> {
        self.repository.update(person_uid, payload).await
    }

    async fn update_last_synced_at(
        &self,
        person_uid: Uuid,
        last_synced_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<CalendarSubscription> {
        self.repository
            .update_last_synced_at(person_uid, last_synced_at)
            .await
    }

    async fn delete_subscription(&self, person_uid: Uuid) -> Result<()> {
        self.repository.delete(person_uid).await
    }
}
