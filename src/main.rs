use rocket_sync_db_pools::{database, diesel::PgConnection};

#[macro_use]
extern crate rocket;

pub mod handlers;
pub mod models;
pub mod schema;

use handlers::{
    chat::{create_chat, delete_chat, get_chat, list_chats, update_chat},
    conversation::{inference,inference_internet,create_conversation},
};

#[database("llama_chat")]
pub struct DbConn(PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/chats",
            routes![create_chat, delete_chat, get_chat, list_chats, update_chat],
        )
        .mount("/conversations", routes![inference,inference_internet,create_conversation])
        .attach(DbConn::fairing())
}
