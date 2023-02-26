use anyhow::Result;

mod discord;
mod logger;
mod openai;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    logger::create_logger("bot.log")?;

    // let resp = get_openai_text("Write a haiku".to_string()).await?;
    // log::info!("{resp}");

    discord::run().await?;

    Ok(())
}
