use std::{env, str::FromStr};

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn get_openai_text(prompt: String, temp: f32) -> Result<String> {
    let req = OpenAiRequest {
        model: "text-davinci-003".to_string(),
        temperature: Some(temp),
        prompt,
        max_tokens: 60,
    };

    let mut auth = String::from_str("Bearer ")?;
    auth.push_str(&env::var("OPENAI_API_KEY")?);

    let client = Client::new();
    let res = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", auth)
        .json(&req)
        .send()
        .await?;

    let content: OpenAiResponse = res.json().await?;

    Ok(content
        .choices
        .first()
        .ok_or(anyhow!("choice not available"))?
        .text
        .clone())
}

#[derive(Serialize, Debug)]
struct OpenAiRequest {
    model: String,
    temperature: Option<f32>,
    prompt: String,
    max_tokens: u16,
}

#[derive(Deserialize, Debug)]
struct GptTextOption {
    text: String,
    index: u32,
    logprobs: Option<u32>,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
struct UsageDetails {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct OpenAiResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<GptTextOption>,
    usage: UsageDetails,
}
