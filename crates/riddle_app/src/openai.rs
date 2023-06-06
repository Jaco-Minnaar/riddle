use std::str::FromStr;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub struct OpenAiClient {
    http_client: reqwest::Client,
    api_key: String,
}

impl OpenAiClient {
    pub fn new(http_client: reqwest::Client, api_key: String) -> Self {
        Self {
            http_client,
            api_key,
        }
    }

    pub async fn get_openai_text(&self, prompt: String, temp: f32) -> Result<String> {
        let req = OpenAiRequest {
            model: "text-davinci-003".to_string(),
            temperature: Some(temp),
            prompt: Some(prompt),
            // messages: None,
            max_tokens: 1000,
        };

        dbg!(&req);

        let mut auth = String::from_str("Bearer ")?;
        auth.push_str(&self.api_key);

        let res = self
            .http_client
            .post("https://api.openai.com/v1/completions")
            .header("Authorization", auth)
            .json(&req)
            .send()
            .await?;

        dbg!(res.status());

        let content: OpenAiResponse = res.json().await?;

        dbg!(&content);

        match content {
            OpenAiResponse::Ok { choices, .. } => {
                let first_choice = choices.first().ok_or(anyhow!("choice not available"))?;

                if let Some(text) = &first_choice.text {
                    return Ok(text.to_string());
                }

                if let Some(messages) = &first_choice.message {
                    return Ok(messages.content.clone());
                }

                Err(anyhow!("choice not available"))
            }
            OpenAiResponse::Err { error } => Err(anyhow!(error.message)),
        }
    }
}

#[derive(Debug, Deserialize)]
struct OpenAiError {
    message: String,
    r#type: String,
    param: Option<()>,
    code: Option<()>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum OpenAiResponse {
    Ok {
        id: String,
        object: String,
        created: u64,
        model: String,
        choices: Vec<GptTextOption>,
        usage: UsageDetails,
    },
    Err {
        error: OpenAiError,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum ChatRole {
    System,
    Assistant,
    User,
}

#[derive(Serialize, Debug)]
struct OpenAiRequest {
    model: String,
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<String>,
    // messages: Option<Vec<GptChatMessage>>,
    max_tokens: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct GptChatMessage {
    role: ChatRole,
    content: String,
}

#[derive(Deserialize, Debug)]
struct GptTextOption {
    text: Option<String>,
    message: Option<GptChatMessage>,
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
