use anyhow::anyhow;
use shuttle_secrets::SecretStore;

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
