use rocket::response::Debug;

pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

pub mod chat;
pub mod conversation;