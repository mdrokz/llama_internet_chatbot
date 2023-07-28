use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewConversation {
    pub chat_id: Uuid,
    pub role: String,
    pub message: String
}