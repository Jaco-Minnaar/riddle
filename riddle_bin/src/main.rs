mod logger;

use anyhow::Result;
use riddle::app;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    logger::create_logger("riddle.log").expect("error creating logger");

    let discord_token = env::var("DISCORD_API_KEY")?;
    let openai_token = env::var("OPENAI_API_KEY")?;

    let mut client = app::init_client(&discord_token, openai_token).await?;

    if let Err(e) = client.start().await {
        log::error!("Error: {}", e);
    }

    Ok(())
}
