use crate::core::models::calendar::{CalendarSubscription, CreateCalendarSubscription};
use crate::core::repositories::calendar::CalendarSubscriptionRepository;
use crate::infrastructure::databases::postgres::DBPool;
use crate::prelude::*;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::sync::Arc;
use uuid::Uuid;

pub struct CalendarSubscriptionRepositoryImpl {
    pub pool: Arc<DBPool>,
}

impl CalendarSubscriptionRepositoryImpl {
    pub fn new(pool: Arc<DBPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CalendarSubscriptionRepository for CalendarSubscriptionRepositoryImpl {
    async fn create(
        &self,
        calendar_subscription: CreateCalendarSubscription,
    ) -> Result<CalendarSubscription> {
        // delete old subscriptions
        let mut conn = self.pool.get()?;
        let _ = diesel::delete(crate::schema::calendar_subscriptions::table)
            .filter(
                crate::schema::calendar_subscriptions::person_uid
                    .eq(calendar_subscription.person_uid),
            )
            .execute(&mut conn);

        // insert new subscription
        diesel::insert_into(crate::schema::calendar_subscriptions::table)
            .values(&calendar_subscription)
            .returning(crate::schema::calendar_subscriptions::all_columns)
            .get_result(&mut conn)
            .map_err(|e| e.into())
    }

    async fn get(&self, secret: &String) -> Result<CalendarSubscription> {
        let mut conn = self.pool.get()?;
        crate::schema::calendar_subscriptions::table
            .filter(crate::schema::calendar_subscriptions::secret.eq(secret))
            .first(&mut conn)
            .map_err(|e| e.into())
    }

    async fn get_person_subscription(&self, person_uid: Uuid) -> Result<CalendarSubscription> {
        let mut conn = self.pool.get()?;
        crate::schema::calendar_subscriptions::table
            .filter(crate::schema::calendar_subscriptions::person_uid.eq(person_uid))
            .first(&mut conn)
            .map_err(|e| e.into())
    }

    async fn delete(&self, person_uid: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;
        diesel::delete(crate::schema::calendar_subscriptions::table)
            .filter(crate::schema::calendar_subscriptions::person_uid.eq(person_uid))
            .execute(&mut conn)
            .map_err(|e| e.into())
            .map(|_| ())
    }
}
