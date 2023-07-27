use chrono::Local;
use diesel::{Insertable,Queryable};
use serde::{Serialize,Deserialize};
use super::schema::{chats,conversations};

#[derive(Queryable,Insertable,Clone,Debug,Serialize,Deserialize)]
#[diesel(table_name = chats)]
pub struct Chat {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable,Insertable,Clone,Debug,Serialize,Deserialize)]
#[diesel(table_name = conversations)]
pub struct Conversation {
    pub id: uuid::Uuid,
    pub role: String,
    pub chat_id: uuid::Uuid,
    pub message: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: String::new(),
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}

impl Default for Conversation {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            role: String::new(),
            chat_id: uuid::Uuid::new_v4(),
            message: String::new(),
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        }
    }
}