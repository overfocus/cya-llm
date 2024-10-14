use anyhow::{Result, anyhow};
use async_trait::async_trait;
use reqwest::{Client, Url};
use serde_json::json;
use tokio::time::{sleep, Duration, Instant};
use std::env;

use super::client::LlmClient;

pub struct HuggingFaceLlmClient {
    client: Client,
    api_key: String,
    model_url: Url,
}

impl HuggingFaceLlmClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("HUGGINGFACE_API_KEY")
            .expect("HUGGINGFACE_API_KEY must be set");
        
        let model_address = env::var("HUGGINGFACE_MODEL")
            .unwrap_or_else(|_| "EleutherAI/gpt-neo-2.7B".to_string());
        
        let model_url = Url::parse(&format!(
            "https://api-inference.huggingface.co/models/{}",
            model_address
        ))?;

        Ok(Self {
            client: Client::new(),
            api_key,
            model_url,
        })
    }


    async fn make_request(&self, prompt: &str) -> Result<String> {
        let max_retries = 10; 
        let mut delay = Duration::from_secs(2);
        let start_time = Instant::now();

        println!("Using {}",self.model_url.clone());

        for attempt in 1..=max_retries {
            let response = self.client
                .post(self.model_url.clone())
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&json!({
                    "inputs": prompt,
                    "parameters": {
                        "max_new_tokens": 300,
                        "temperature": 0.7,
                        "top_p": 0.95,
                        "do_sample": true,
                        "return_full_text": true
                    }
                }))
                .send()
                .await?;

            let status = response.status();

            if status.is_success() {
                let response_json: serde_json::Value = response.json().await?;

                println!("FULL RESPONSE {} \r\n\r\n",response_json.clone());
                return Ok(response_json[0]["generated_text"]
                    .as_str()
                    .ok_or_else(|| anyhow!("Failed to parse response"))?
                    .to_string());
            } else if status == reqwest::StatusCode::SERVICE_UNAVAILABLE {
                let error_body: serde_json::Value = response.json().await?;
                if let Some(estimated_time) = error_body["estimated_time"].as_f64() {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let adjusted_estimate = (estimated_time - elapsed).max(0.0);
                    println!(
                        "Model is loading. Adjusted estimate: {:.2} seconds. Attempt {}/{}. Elapsed: {:.2}s",
                        adjusted_estimate, attempt, max_retries, elapsed
                    );
                    sleep(delay).await;
                    delay = delay.mul_f32(1.5); // Slightly slower backoff
                } else {
                    return Err(anyhow!("Unexpected 503 error: {:?}", error_body));
                }
            } else {
                let error_body = response.text().await?;
                return Err(anyhow!("API request failed: {} - {}", status, error_body));
            }
        }

        Err(anyhow!("Max retries reached. Unable to generate story segment."))
    }
}

#[async_trait]
impl LlmClient for HuggingFaceLlmClient {
    async fn generate_story_segment(&self, current_situation: &str) -> Result<String> {
        let prompt = format!(
            "Generate a short story segment where the protagonist {}. \
            End the segment with a question asking the reader what they want to do next. \
            Provide exactly three numbered choices for the reader to choose from.\n\nStory:",
            current_situation
        );

        self.make_request(&prompt).await
    }
}