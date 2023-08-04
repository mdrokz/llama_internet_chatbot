use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    response::{
        status::Created,
        stream::{Event, EventStream},
    },
    serde::json::Json,
};

// use rocket::tokio::sync::mpsc::{channel};
use rocket::tokio::task::spawn_blocking;

// use std::thread::spawn;

use std::sync::mpsc::channel;

use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

use crate::{schema::conversations::chat_id, DbConn};

use self::db::NewConversation;

pub mod db;

use super::Result;

#[post("/create", format = "json", data = "<conversation>")]
pub async fn create_conversation(
    conn: DbConn,
    conversation: Json<NewConversation>,
) -> Result<Created<Json<NewConversation>>> {
    use crate::schema::conversations::dsl::*;

    use crate::models::Conversation;

    let new_conversation = Conversation {
        role: conversation.role.clone(),
        message: conversation.message.clone(),
        ..Default::default()
    };

    conn.run(move |c| {
        diesel::insert_into(conversations)
            .values(&new_conversation)
            .execute(c)
            .expect("Error saving new post")
    })
    .await;

    Ok(Created::new("/").body(conversation))
}

#[get("/inference/<id>")]
pub async fn inference(conn: DbConn, id: uuid::Uuid) -> EventStream![] {
    use crate::schema::conversations::dsl::conversations;

    let (tx, rx) = channel::<String>();

    let messages = conn
        .run(move |c| {
            conversations
                .filter(chat_id.eq(id))
                .limit(20)
                .load::<crate::models::Conversation>(c)
                .expect("Error loading conversations")
        })
        .await;

    if messages.len() == 0 {
        let _ = tx.send("CANCEL".into());
    }

    if messages.len() > 0 {
        let model_options = ModelOptions {
            ..Default::default()
        };

        let predict_options = PredictOptions {
            // n_keep: 10,
            // repeat: 256,
            // batch: 1024,
            tokens: 0,
            threads: 14,
            top_k: 90,
            top_p: 0.86,
            // penalty: 1.17647,
            stop_prompts: vec!["Human:".into()],
            token_callback: Some(Box::new(move |e| {
                let _ = tx.send(e);
                true
            })),
            ..Default::default()
        };
        spawn_blocking(move || {
            let llama = LLama::new("./wizard-vicuna-13B.ggmlv3.q4_0.bin".into(), &model_options)
                .expect("failed to create model");
            // let r = "### ### Human: Hello, ### Assistant.\n### ### Assistant: Hello. How may I help you today?\n### ### Human: Please tell me the largest city in Europe.\n### ### Assistant: Sure. The largest city in Europe is Moscow, the capital of Russia.\n### ### Human: whats the first question i asked ?";
            let prompt = messages
                .iter()
                .map(|m| format!("[[{}]]: {}", m.role, m.message))
                .collect::<Vec<String>>()
                .join("\n");

            llama
                .predict(prompt, predict_options)
                .expect("failed to predict");
        });
    }

    EventStream! {
        loop {
            let token = rx.recv();
            match token {
                Ok(token) => {
                    if token == "CANCEL" {
                        yield Event::data("No messages found in database");
                        println!("shutting down inference");
                        break;
                    }
                    println!("TOKEN: {}", token);
                    yield Event::data(token);
                }
                Err(e) => {
                    println!("{:?}", e);
                    break;
                }
            }
        }
    }
}
