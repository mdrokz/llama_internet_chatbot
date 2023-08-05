use rocket_sync_db_pools::{database, diesel::PgConnection};

#[macro_use]
extern crate rocket;

pub mod handlers;
pub mod models;
pub mod schema;
pub mod utils;

use fern::Dispatch;
use log::LevelFilter;
use rocket::serde::json::serde_json::json;

use handlers::{
    chat::{create_chat, delete_chat, get_chat, list_chats, update_chat},
    conversation::{create_conversation, inference, inference_internet, list_conversations},
};

#[database("llama_chat")]
pub struct DbConn(PgConnection);

fn setup_logger() -> Result<(), fern::InitError> {
    let file = fern::log_file("logs/output.log")?;
    Dispatch::new()
        .format(|out, message, record| {
            let log = json!({
                "timestamp": chrono::Local::now().to_rfc3339(),
                "level": record.level().to_string(),
                "message": message.to_string(),
            });

            out.finish(format_args!("{}", log))
            // out.finish(format_args!(
            //     "{} [{}] {}",
            //     chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            //     format!("{}", record.level()).color(color),
            //     message
            // ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(file)
        .apply()?;
    Ok(())
}

#[launch]
fn rocket() -> _ {

    let _ = setup_logger();

    rocket::build()
        .mount(
            "/chats",
            routes![create_chat, delete_chat, get_chat, list_chats, update_chat],
        )
        .mount(
            "/conversations",
            routes![
                inference,
                inference_internet,
                create_conversation,
                list_conversations
            ],
        )
        .attach(DbConn::fairing())
}
