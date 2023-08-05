use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use reqwest::{
    header::{HeaderMap, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
    redirect::Policy,
};
use rocket::{
    response::{
        status::Created,
        stream::{Event, EventStream},
    },
    serde::json::Json,
};

// use rocket::tokio::sync::mpsc::{channel};
use rocket::tokio::task::spawn_blocking;
use visdom::Vis;

// use std::thread::spawn;

use std::sync::mpsc::channel;

use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

use crate::{models::Conversation, schema::conversations::chat_id, DbConn};

use self::db::NewConversation;

pub mod db;

use super::Result;

const UA: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36";

const ACCEPT_VALUE: &'static str =
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8";

const ACCEPT_ENCODING_VALUE: &'static str = "gzip, deflate, br";

const ACCEPT_LANGUAGE_VALUE: &'static str = "en-US,en;q=0.9,lt;q=0.8,et;q=0.7,de;q=0.6";

#[post("/create", format = "json", data = "<conversation>")]
pub async fn create_conversation(
    conn: DbConn,
    conversation: Json<NewConversation>,
) -> Result<Created<Json<NewConversation>>> {
    use crate::schema::conversations::dsl::*;

    use crate::models::Conversation;

    let initial_message_h = Conversation {
        role: "Human".into(),
        chat_id: conversation.chat_id.clone(),
        message: "Hello, [[Assistant]].".into(),
        ..Default::default()
    };

    let initial_message_a = Conversation {
        role: "Assistant".into(),
        chat_id: conversation.chat_id.clone(),
        message: "ello. How may I help you today?".into(),
        ..Default::default()
    };

    let new_conversation = Conversation {
        role: conversation.role.clone(),
        chat_id: conversation.chat_id.clone(),
        message: conversation.message.clone(),
        ..Default::default()
    };

    conn.run(move |c| {
        diesel::insert_into(conversations)
            .values(vec![
                initial_message_h,
                initial_message_a,
                new_conversation,
            ])
            .execute(c)
            .expect("Error saving new post")
    })
    .await;

    Ok(Created::new("/").body(conversation))
}

#[get("/", format = "json")]
pub async fn list_conversations(conn: DbConn) -> Result<Json<Vec<Conversation>>> {
    use crate::schema::conversations::dsl::*;

    let conversations_list = conn
        .run(|c| {
            conversations
                .limit(100)
                .load::<Conversation>(c)
                .expect("Error loading conversations")
        })
        .await;

    Ok(Json(conversations_list))
}

#[get("/inference_internet/<id>")]
pub async fn inference_internet(conn: DbConn, id: uuid::Uuid) -> EventStream![] {
    use crate::schema::conversations::dsl::conversations;

    let mut headers = HeaderMap::new();

    headers.append(USER_AGENT, UA.parse().unwrap());

    headers.append(ACCEPT, ACCEPT_VALUE.parse().unwrap());

    headers.append(ACCEPT_ENCODING, ACCEPT_ENCODING_VALUE.parse().unwrap());

    headers.append(ACCEPT_LANGUAGE, ACCEPT_LANGUAGE_VALUE.parse().unwrap());

    let client = reqwest::Client::builder()
        .redirect(Policy::default())
        .default_headers(headers)
        .build()
        .expect("failed to build client");

    let (tx, rx) = channel::<String>();

    let mut messages = conn
        .run(move |c| {
            conversations
                .filter(chat_id.eq(id))
                .limit(100)
                .load::<crate::models::Conversation>(c)
                .expect("Error loading conversations")
        })
        .await;

    if messages.len() == 0 {
        let _ = tx.send("CANCEL".into());
    }

    let query = messages.last().unwrap().message.clone();

    if messages.len() > 0 {
        let res = client
            .get(format!("https://duckduckgo.com/html/?q={}", &query))
            .send()
            .await
            .expect("failed to send duckduckgo request");

        // println!("{:?}", res);

        let body = res.text().await.expect("failed to extract body");

        println!("{:?}", body);

        let links = {
            let root = Vis::load(body).unwrap();

            let b = root.find(".result__title>a").slice(0..10);

            b.map(|_, e| {
                let href = e.get_attribute("href").unwrap();
                href.to_string()
            }) // Collect the links into a Vec
        };

        let mut docs = vec![];

        if links.len() == 0 {
            let _ = tx.send("CANCEL".into());
        } else {
            for link in links {
                println!("{}", link);

                let r = client
                    .get(link)
                    .send()
                    .await
                    .expect("failed to send request");

                let body = r.text().await.expect("failed to extract body");

                let (p, b, s) = {
                    let root = Vis::load(body).unwrap();

                    let p = root.find("p").slice(0..5).text();

                    let b = root.find("tr").slice(0..5).text();

                    let s = root.find("span").slice(0..5).text();

                    (p, b, s)
                };

                // combine p b s
                let text = format!("{}\n\n {}\n\n {}\n\n", p, b, s);

                docs.push(text);
            }

            let rank_res = client
                .post(format!("http://localhost:8081/rank?query={}", &query))
                .json(&docs)
                .send()
                .await
                .expect("failed to send request");

            let rank = rank_res
                .text()
                .await
                .expect("failed to extract body")
                .parse::<usize>()
                .expect("failed to parse rank");

            println!("rank: {}", rank);

            messages.push(Conversation {
                role: "Fetched Information".into(),
                message: docs[rank].clone(),
                ..Default::default()
            });

            messages.push(Conversation {
                role: "Human".into(),
                message: "Please understand the [[Fetched Information]] and give an answer to me [[Assistant]].".into(),
                ..Default::default()
            });

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
                let llama =
                    LLama::new("./models/wizard-vicuna-13B.ggmlv3.q4_0.bin".into(), &model_options)
                        .expect("failed to create model");
                // let r = "### ### Human: Hello, ### Assistant.\n### ### Assistant: Hello. How may I help you today?\n### ### Human: Please tell me the largest city in Europe.\n### ### Assistant: Sure. The largest city in Europe is Moscow, the capital of Russia.\n### ### Human: whats the first question i asked ?";
                let prompt = messages
                    .iter()
                    .map(|m| format!("[[{}]]: {}", m.role, m.message))
                    .collect::<Vec<String>>()
                    .join("\n");

                println!("{}", prompt);

                llama
                    .predict(prompt, predict_options)
                    .expect("failed to predict");
            });
        }
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

#[get("/inference/<id>")]
pub async fn inference(conn: DbConn, id: uuid::Uuid) -> EventStream![] {
    use crate::schema::conversations::dsl::conversations;

    let (tx, rx) = channel::<String>();

    let messages = conn
        .run(move |c| {
            conversations
                .filter(chat_id.eq(id))
                .limit(100)
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
            let llama = LLama::new("./models/wizard-vicuna-13B.ggmlv3.q4_0.bin".into(), &model_options)
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
