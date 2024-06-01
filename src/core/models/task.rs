use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct Task {
    pub uid: Uuid,
    pub summary: String,
    pub description: String,
    pub completed: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
    pub author_uid: Uuid,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct CreateTask {
    pub summary: String,
    pub author_uid: Uuid,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::tasks)]
pub struct UpdateTask {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub completed: Option<DateTime<Utc>>,
}
