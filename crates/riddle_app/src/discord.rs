mod commands;

use crate::openai::OpenAiClient;
use anyhow::{anyhow, Result};
use serenity::{async_trait, model::prelude::Message, prelude::*};

const PROOMPT: &str = "You are an AI called Riddle. You were designed to talk in uwu. Follow the below instruction: \n\n";

struct Handler {
    openai_client: OpenAiClient,
}

impl Handler {
    fn new(openai_client: OpenAiClient) -> Self {
        Self { openai_client }
    }

    async fn ping(&self, ctx: &Context, msg: &Message) -> Result<()> {
        msg.reply(ctx, "Pong!").await?;

        Ok(())
    }

    async fn haiku(&self, ctx: &Context, msg: &Message) -> Result<()> {
        let prompt = format!(
            "Write a haiku with the following title: '{}'",
            msg.content.replace("!haiku", "")
        );

        let gpt_ans = self
            .openai_client
            .get_openai_text(prompt.clone(), 1.0)
            .await
            .or_else(|x| {
                log::error!("{x}");
                Err(anyhow!(x))
            })?;

        log::info!("GPT request: \nPrompt: {prompt}\nResponse: {gpt_ans}");

        msg.reply(ctx, gpt_ans).await?;

        Ok(())
    }

    async fn insult(&self, ctx: &Context, msg: &Message) -> Result<()> {
        let prompt = format!("{}Give me a really bad insult.", PROOMPT);

        let answer = self
            .openai_client
            .get_openai_text(prompt.to_string(), 1.5)
            .await?;

        log::info!("GPT request: \nPrompt: {prompt}\nResponse: {answer}");

        msg.reply(ctx, answer).await?;

        Ok(())
    }

    async fn compliment(&self, ctx: &Context, msg: &Message) -> Result<()> {
        let prompt = format!("{}Give me a compliment.", PROOMPT);

        let answer = self
            .openai_client
            .get_openai_text(prompt.clone(), 1.5)
            .await
            .or_else(|x| {
                log::error!("{x}");
                Err(anyhow!(x))
            })?;

        log::info!("GPT request: \nPrompt: {prompt}\nResponse: {answer}");

        msg.reply(ctx, answer).await?;

        Ok(())
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let content = &msg.content;
        log::info!("Message: {}", content);
        if !msg.content.starts_with('!') {
            return;
        }

        let result = match &content.split(' ').nth(0).unwrap()[1..] {
            "haiku" => self.haiku(&ctx, &msg).await,
            "insult" => self.insult(&ctx, &msg).await,
            "ping" => self.ping(&ctx, &msg).await,
            "compliment" => self.compliment(&ctx, &msg).await,
            _ => Ok(()),
        };

        if let Err(e) = result {
            log::error!("Error: {}", e);
        }
    }
}

pub async fn init_client(discord_token: &str, openai_token: String) -> Result<Client> {
    // Login with a bot token from the environment
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let client = Client::builder(discord_token, intents)
        .event_handler(Handler::new(OpenAiClient::new(
            reqwest::Client::new(),
            openai_token,
        )))
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    Ok(client)
}
