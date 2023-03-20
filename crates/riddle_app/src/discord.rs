mod commands;

use std::{env, str::FromStr};

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

const PROOMPT: &str = "You are an AI called Riddle. You were designed to talk in riddles and uwu. Follow the below instruction: \n\n";

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
    let prompt = format!("{}Give me a really bad insult.", PROOMPT);

    let answer = get_openai_text(prompt.to_string(), 1.5).await?;

    log::info!("GPT request: \nPrompt: {prompt}\nResponse: {answer}");

    msg.reply(ctx, answer).await?;

    Ok(())
}

#[command]
async fn compliment(ctx: &Context, msg: &Message) -> CommandResult {
    let prompt = format!("{}Give me a compliment.", PROOMPT);

    let answer = get_openai_text(prompt.clone(), 1.5).await.or_else(|x| {
        log::error!("{x}");
        Err(anyhow!(x))
    })?;

    log::info!("GPT request: \nPrompt: {prompt}\nResponse: {answer}");

    msg.reply(ctx, answer).await?;

    Ok(())
}

pub async fn init_client() -> Result<Client> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_API_KEY").or(Err(anyhow!("Discord token not found")))?;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    Ok(client)
}
