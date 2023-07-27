use std::env;

// use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

use rocket_sync_db_pools::{diesel::PgConnection, database};

#[macro_use]
extern crate rocket;

pub mod handlers;
pub mod models;
pub mod schema;

use handlers::chat::{create_chat, delete_chat, get_chat, list_chats, update_chat};

#[database("llama_chat")]
pub struct DbConn(PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/chats",
        routes![create_chat, delete_chat, get_chat, list_chats, update_chat],
    ).attach(DbConn::fairing())
}
