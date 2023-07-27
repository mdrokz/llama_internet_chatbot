use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewChat {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateChat {
    pub name: String,
    pub updated_at: chrono::NaiveDateTime,
}
