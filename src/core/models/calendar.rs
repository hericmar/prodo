use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::calendar_subscriptions)]
pub struct CalendarSubscription {
    pub secret: String,
    pub person_uid: Uuid,
    pub timezone: String,
    pub last_synced_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::calendar_subscriptions)]
pub struct CreateCalendarSubscription {
    pub secret: String,
    pub person_uid: Uuid,
    pub timezone: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::calendar_subscriptions)]
pub struct UpdateCalendarSubscription {
    pub timezone: Option<String>,
}
