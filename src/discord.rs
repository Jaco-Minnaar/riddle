mod commands;

use std::env;

use crate::openai::get_openai_text;
use anyhow::{anyhow, Result};
use serenity::{
    async_trait,
    framework::{
        standard::{
            macros::{command, group},
            CommandResult,
        },
        StandardFramework,
    },
    model::prelude::Message,
    prelude::*,
};

#[group]
#[commands(ping, haiku, insult, compliment)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn haiku(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt = format!(
        "Write a haiku with the following title: '{}'",
        msg.content.replace("!haiku", "")
    );

    let gpt_ans = get_openai_text(prompt.clone(), 1.0).await.or_else(|x| {
        log::error!("{x}");
        Err(anyhow!(x))
    })?;

    log::info!("GPT request: \nPrompt: {prompt}\nResponse: {gpt_ans}");

    msg.reply(ctx, gpt_ans).await?;

    Ok(())
}

#[command]
async fn insult(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt = "think of a random insult";

    let answer = get_openai_text(prompt.to_string(), 1.5).await?;

    log::info!("GPT request: \nPrompt: {prompt}\nResponse: {answer}");

    msg.reply(ctx, answer).await?;

    Ok(())
}

#[command]
async fn compliment(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt = "give me a compliment";

    let answer = get_openai_text(prompt.to_string(), 1.5)
        .await
        .or_else(|x| {
            log::error!("{x}");
            Err(anyhow!(x))
        })?;

    log::info!("GPT request: \nPrompt: {prompt}\nResponse: {answer}");

    msg.reply(ctx, answer).await?;

    Ok(())
}

pub async fn run() -> Result<()> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_API_KEY").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
        Err(anyhow!(why))
    } else {
        Ok(())
    }
}
