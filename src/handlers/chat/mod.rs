use crate::{establish_connection_pg, models::Chat};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    response::{status::Created, Debug},
    serde::json::Json,
};

use self::db::{NewChat, UpdateChat};

mod db;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/create", format = "json", data = "<chat>")]
pub fn create_chat(chat: Json<NewChat>) -> Result<Created<Json<NewChat>>> {
    use crate::schema::chats::dsl::*;

    use crate::models::Chat;

    let connection = &mut establish_connection_pg();

    let new_chat = Chat {
        name: chat.name.clone(),
        ..Default::default()
    };

    diesel::insert_into(chats)
        .values(&new_chat)
        .execute(connection)
        .expect("Error saving new post");

    Ok(Created::new("/").body(chat))
}

#[get("/", format = "json")]
pub fn list_chats() -> Result<Json<Vec<Chat>>> {
    use crate::schema::chats::dsl::*;

    let connection = &mut establish_connection_pg();

    let chats_list = chats
        .limit(20)
        .load::<Chat>(connection)
        .expect("Error loading chats");

    Ok(Json(chats_list))
}

#[get("/<id>", format = "json")]
pub fn get_chat(id: uuid::Uuid) -> Result<Json<Chat>> {
    use crate::schema::chats::dsl::chats;

    let connection = &mut establish_connection_pg();

    let chat = chats
        .find(id)
        .first::<Chat>(connection)
        .expect("Error loading chat");

    Ok(Json(chat))
}

#[put("/update/<id>", format = "json", data = "<chat>")]
pub fn update_chat(id: uuid::Uuid, chat: Json<UpdateChat>) -> Result<Json<Chat>> {
    use crate::schema::chats::dsl::{chats, name};

    let connection = &mut establish_connection_pg();

    let chat = diesel::update(chats.find(id))
        .set(name.eq(chat.name.clone()))
        .get_result::<Chat>(connection)
        .expect("Error updating chat");

    Ok(Json(chat))
}

#[delete("/delete/<id>", format = "json")]
pub fn delete_chat(id: uuid::Uuid) -> Result<Json<Chat>> {
    use crate::schema::chats::dsl::chats;

    let connection = &mut establish_connection_pg();

    let chat = diesel::delete(chats.find(id))
        .get_result::<Chat>(connection)
        .expect("Error deleting chat");

    Ok(Json(chat))
}
