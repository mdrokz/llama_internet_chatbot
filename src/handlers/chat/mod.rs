use crate::{models::Chat, DbConn};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    response::{status::Created, Debug},
    serde::json::Json,
};

use self::db::{NewChat, UpdateChat};

mod db;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/create", format = "json", data = "<chat>")]
pub async fn create_chat(conn: DbConn, chat: Json<NewChat>) -> Result<Created<Json<NewChat>>> {
    use crate::schema::chats::dsl::*;

    use crate::models::Chat;

    let new_chat = Chat {
        name: chat.name.clone(),
        ..Default::default()
    };

    conn.run(move |c| {
        diesel::insert_into(chats)
            .values(&new_chat)
            .execute(c)
            .expect("Error saving new post")
    })
    .await;

    Ok(Created::new("/").body(chat))
}

#[get("/", format = "json")]
pub async fn list_chats(conn: DbConn) -> Result<Json<Vec<Chat>>> {
    use crate::schema::chats::dsl::*;

    let chats_list = conn
        .run(|c| {
            chats
                .limit(20)
                .load::<Chat>(c)
                .expect("Error loading chats")
        })
        .await;

    Ok(Json(chats_list))
}

#[get("/<id>", format = "json")]
pub async fn get_chat(conn: DbConn, id: uuid::Uuid) -> Result<Json<Chat>> {
    use crate::schema::chats::dsl::chats;

    let chat = conn
        .run(move |c| chats.find(id).first::<Chat>(c).expect("Error loading chat"))
        .await;

    Ok(Json(chat))
}

#[put("/update/<id>", format = "json", data = "<chat>")]
pub async fn update_chat(
    conn: DbConn,
    id: uuid::Uuid,
    chat: Json<UpdateChat>,
) -> Result<Json<Chat>> {
    use crate::schema::chats::dsl::{chats, name};

    let chat = conn
        .run(move |c| {
            diesel::update(chats.find(id))
                .set(name.eq(chat.name.clone()))
                .get_result::<Chat>(c)
                .expect("Error updating chat")
        })
        .await;

    Ok(Json(chat))
}

#[delete("/delete/<id>", format = "json")]
pub async fn delete_chat(conn: DbConn, id: uuid::Uuid) -> Result<Json<Chat>> {
    use crate::schema::chats::dsl::chats;

    let chat = conn
        .run(move |c| {
            diesel::delete(chats.find(id))
                .get_result::<Chat>(c)
                .expect("Error deleting chat")
        })
        .await;

    Ok(Json(chat))
}
