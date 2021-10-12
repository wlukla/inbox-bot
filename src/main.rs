use dotenv::dotenv;
use std::env;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    teloxide::enable_logging!();
    log::info!("Starting inbox bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        message.answer("pong!").await?;
        respond(())
    })
    .await;
}
