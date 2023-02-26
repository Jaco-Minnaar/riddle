use std::env;

use anyhow::{anyhow, Result};
use shuttle_secrets::SecretStore;

mod discord;
mod logger;
mod openai;

const DISCORD_KEY: &'static str = "DISCORD_API_KEY";
const OPENAI_KEY: &'static str = "OPENAI_API_KEY";

#[shuttle_service::main]
async fn riddle(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // dotenv::dotenv().ok();
    // logger::create_logger("riddle.log").expect("error creating logger");

    let discord_token = secret_store
        .get(DISCORD_KEY)
        .ok_or(anyhow!("discord API key not found"))?;
    let openai_token = secret_store
        .get(OPENAI_KEY)
        .ok_or(anyhow!("OpenAI API key not found"))?;

    env::set_var(DISCORD_KEY, discord_token);
    env::set_var(OPENAI_KEY, openai_token);

    let client = discord::init_client().await?;

    Ok(client)
}
