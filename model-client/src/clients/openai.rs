use async_trait::async_trait;

use crate::{
    error::Result,
    traits::{ModelClient, TextGeneration},
    types::text::{TextPrompt, TextResponse},
};

pub struct OpenAIClient {
    pub api_base: String,
    pub api_key: String,
    pub model: String,
}

#[async_trait]
impl ModelClient for OpenAIClient {
    type Input = TextPrompt;
    type Output = TextResponse;

    async fn infer(&self, input: Self::Input) -> Result<Self::Output> {
        // TODO: HTTP 调用逻辑放这里
        Ok(TextResponse {
            content: format!("模拟回答：{}", input.prompt),
            raw: None,
        })
    }
}

impl TextGeneration for OpenAIClient {}
