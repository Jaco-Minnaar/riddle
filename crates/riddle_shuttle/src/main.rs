use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let Some(discord_token) = secret_store.get("DISCORD_API_KEY") else {
        return Err(anyhow!("'DISCORD_API_KEY' was not found").into());
    };

    let Some(openai_token) = secret_store.get("OPENAI_API_KEY") else {
        return Err(anyhow!("'OPENAI_API_KEY' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let client = riddle_app::init_client(&discord_token, openai_token)
        .await
        .expect("Error creating client");

    Ok(client.into())
}
