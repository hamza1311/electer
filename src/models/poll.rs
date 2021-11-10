use crate::models::User;
use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use uuid::Uuid;

fn serialize_user<S>(user: &User, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    user.id.serialize(serializer)
}

#[derive(Debug, Clone, Serialize)]
pub struct Poll {
    pub id: Uuid,
    pub title: String,
    #[serde(serialize_with = "serialize_user")]
    pub creator: User,
    pub create_time: DateTime<Utc>,
}
