use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::client::LlmClient;

pub struct AnthropicLlmClient {
    client: Client,
    api_key: String,
}

impl AnthropicLlmClient {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .expect("ANTHROPIC_API_KEY must be set");
        
        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }
}

#[async_trait]
impl LlmClient for AnthropicLlmClient {
    async fn generate_story_segment(&self, current_situation: &str) -> Result<String> {
        let prompt = format!(
            "Generate a short story segment where the protagonist {}. \
            End the segment with a question asking the reader what they want to do next. \
            Provide exactly three numbered choices for the reader to choose from.",
            current_situation
        );

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&json!({
                "model": "claude-3-opus-20240229",
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": 300
            }))
            .send()
            .await?;

        let response_json: serde_json::Value = response.json().await?;
        let content = response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse response"))?;

        Ok(content.to_string())
    }
}