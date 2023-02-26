use anyhow::Result;

mod discord;
mod logger;
mod openai;

#[shuttle_service::main]
async fn riddle() -> shuttle_service::ShuttleSerenity {
    dotenv::dotenv().ok();
    logger::create_logger("riddle.log").expect("error creating logger");

    let client = discord::init_client().await.unwrap();

    Ok(client)
}
