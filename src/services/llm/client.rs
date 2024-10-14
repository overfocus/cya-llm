use anyhow::Result;
use async_trait::async_trait;

use super::huggingface::HuggingFaceLlmClient;
use super::anthropic::AnthropicLlmClient;

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate_story_segment(&self, current_situation: &str) -> Result<String>;
}

pub enum LlmClientType {
    HuggingFace,
    Anthropic,
}

pub struct LlmFactory;

impl LlmFactory {
    pub fn create_client(client_type: LlmClientType) -> Result<Box<dyn LlmClient>> {
        match client_type {
            LlmClientType::HuggingFace => Ok(Box::new(HuggingFaceLlmClient::new()?)),
            LlmClientType::Anthropic => Ok(Box::new(AnthropicLlmClient::new()?)),
        }
    }
}