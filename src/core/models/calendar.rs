use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::calendar_subscriptions)]
pub struct CalendarSubscription {
    pub secret: String,
    pub person_uid: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::calendar_subscriptions)]
pub struct CreateCalendarSubscription {
    pub secret: String,
    pub person_uid: Uuid,
}
