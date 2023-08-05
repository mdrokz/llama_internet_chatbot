use crate::{log_error, models::Chat, schema::chats::updated_at, DbConn};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::info;
use rocket::{response::status::Created, serde::json::Json};

use self::db::{NewChat, UpdateChat};

use super::Result;

mod db;

#[post("/create", format = "json", data = "<chat>")]
pub async fn create_chat(conn: DbConn, chat: Json<NewChat>) -> Result<Created<Json<NewChat>>> {
    use crate::schema::chats::dsl::*;

    use crate::models::Chat;

    info!("Creating chat: {:?}", chat);

    let new_chat = Chat {
        name: chat.name.clone(),
        ..Default::default()
    };

    info!("Saving to database: {:?}", new_chat);

    conn.run(move |c| {
        let v = diesel::insert_into(chats).values(&new_chat).execute(c);

        log_error!(v, "Error saving to database").unwrap()
    })
    .await;

    Ok(Created::new("/").body(chat))
}

#[get("/", format = "json")]
pub async fn list_chats(conn: DbConn) -> Result<Json<Vec<Chat>>> {
    use crate::schema::chats::dsl::*;

    info!("API call: list_chats");

    let chats_list = conn
        .run(|c| {
            let v = chats.limit(20).load::<Chat>(c);

            log_error!(v, "Error loading chats from db").unwrap()
        })
        .await;

    info!("Found {:?} chats", chats_list);

    Ok(Json(chats_list))
}

#[get("/<id>", format = "json")]
pub async fn get_chat(conn: DbConn, id: uuid::Uuid) -> Result<Json<Chat>> {
    use crate::schema::chats::dsl::chats;

    info!("API call: get_chat({:?})", id);

    let chat = conn
        .run(move |c| {
            let v = chats.find(id).first::<Chat>(c);

            log_error!(v, "Error loading chat from db").unwrap()
        })
        .await;

    info!("Found chat: {:?}", chat);

    Ok(Json(chat))
}

#[put("/update/<id>", format = "json", data = "<chat>")]
pub async fn update_chat(
    conn: DbConn,
    id: uuid::Uuid,
    chat: Json<UpdateChat>,
) -> Result<Json<Chat>> {
    use crate::schema::chats::dsl::{chats, name};

    info!("API call: update_chat({:?})", id);

    let chat = conn
        .run(move |c| {
            let v = diesel::update(chats.find(id))
                .set(name.eq(chat.name.clone()))
                .get_result::<Chat>(c);

            log_error!(v, "Error updating chat").unwrap()
        })
        .await;

    info!("Updated chat: {:?}", chat);

    Ok(Json(chat))
}

#[delete("/delete/<id>", format = "json")]
pub async fn delete_chat(conn: DbConn, id: uuid::Uuid) -> Result<Json<Chat>> {
    use crate::schema::chats::dsl::chats;

    info!("API call: delete_chat({:?})", id);

    let chat = conn
        .run(move |c| {
            let v = diesel::delete(chats.find(id)).get_result::<Chat>(c);

            log_error!(v, "Error deleting chat").unwrap()
        })
        .await;

    info!("Deleted chat: {:?}", chat);

    Ok(Json(chat))
}
