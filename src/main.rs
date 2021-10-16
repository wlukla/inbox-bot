use dotenv::dotenv;
use std::env;

use hyper::{body::HttpBody as _, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use teloxide::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let notion_base_url = env::var("NOTION_BASE_URL").unwrap();
    let notion_token = env::var("NOTION_TOKEN").unwrap();
    let notion_database_id = env::var("NOTION_DB_ID").unwrap();

    println!("url: {}, token: {}", notion_base_url, notion_token);
    let url = (format!("{}/v1/databases/{}", notion_base_url, notion_database_id))
        .parse::<hyper::Uri>()
        .unwrap();

    make_request(url).await

    // teloxide::enable_logging!();
    // log::info!("Starting inbox bot...");

    // let bot = Bot::from_env().auto_send();

    // teloxide::repl(bot, |message| async move {
    //     message.answer("pong!").await?;
    //     respond(())
    // })
    // .await;
}

async fn make_request(url: hyper::Uri) -> Result<()> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut auth_header = String::from("Bearer ");
    let notion_token = env::var("NOTION_TOKEN").unwrap();
    auth_header.push_str(notion_token.as_str());

    let request = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("Authorization", auth_header)
        .header("Notion-Version", "2021-08-16")
        .body(Body::from(""))?;

    let mut res = client.request(request).await?;

    println!("Repsonse: {}", res.status());
    println!("Headers: {:#?}", res.headers());

    while let Some(next) = res.data().await {
        let chunk = next?;
        let text = &chunk;
        println!("{:?}", text);
    }

    println!("\n\nDone!");

    Ok(())
}
