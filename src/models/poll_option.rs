use crate::models::Poll;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct PollOption {
    pub id: Uuid,
    #[serde(skip)]
    pub poll: Poll,
    pub text: String,
}
