mod logger;

use riddle::app;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    logger::create_logger("riddle.log").expect("error creating logger");

    let mut client = app::init_client().await?;

    if let Err(e) = client.start().await {
        log::error!("Error: {}", e);
    }

    Ok(())
}
