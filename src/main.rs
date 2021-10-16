extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use std::env;

use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use teloxide::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    teloxide::enable_logging!();
    log::info!("Starting inbox bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        match message.update.text() {
            Some(text) => {
                make_request(String::from(text)).await;
                message.answer("Saved!").await?;

                respond(())
            }
            None => {
                message.answer("Send text message please").await?;

                respond(())
            }
        }
    })
    .await;
}

async fn make_request(content: String) -> Result<()> {
    let notion_base_url = env::var("NOTION_BASE_URL").unwrap();
    let notion_page_id = env::var("NOTION_PAGE_ID").unwrap();
    let url = (format!("{}/v1/blocks/{}/children", notion_base_url, notion_page_id))
        .parse::<hyper::Uri>()
        .unwrap();
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut auth_header = String::from("Bearer ");
    let notion_token = env::var("NOTION_TOKEN").unwrap();
    auth_header.push_str(notion_token.as_str());

    let new_block = Block::new(String::from(content));
    let req_body = serde_json::to_string_pretty(&new_block).unwrap();

    let request = Request::builder()
        .method(Method::PATCH)
        .uri(url)
        .header("Authorization", auth_header)
        .header("Notion-Version", "2021-08-16")
        .header("Content-Type", "application/json")
        .body(Body::from(req_body))?;

    client.request(request).await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Text {
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ParagraphText {
    r#type: String,
    text: Text,
}

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    text: [ParagraphText; 1],
}

#[derive(Serialize, Deserialize, Debug)]
struct Children {
    object: String,
    r#type: String,
    paragraph: Paragraph,
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    children: [Children; 1],
}

impl Block {
    fn new(content: String) -> Block {
        Block {
            children: [Children {
                object: String::from("block"),
                r#type: String::from("paragraph"),
                paragraph: Paragraph {
                    text: [ParagraphText {
                        r#type: String::from("text"),
                        text: Text { content },
                    }],
                },
            }],
        }
    }
}
