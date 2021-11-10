use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    pub create_time: DateTime<Utc>,
}
