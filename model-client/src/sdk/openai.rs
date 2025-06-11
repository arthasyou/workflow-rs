use serde::Serialize;
use service_utils_rs::utils::{ByteStream, Request};

use crate::error::Result;

/// Role in chat messages.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// Single chat message.
#[derive(Debug, Clone, Serialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

/// Request body for chat completion.
#[derive(Debug, Clone, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

/// ChatCompletion client using your wrapped Request.
pub struct OpenAIClient {
    request: Request,
}

impl OpenAIClient {
    pub fn new(api_key: &str, base_url: &str) -> Result<Self> {
        let mut request = Request::new();
        request.set_base_url(base_url)?;
        request.set_default_headers(vec![
            ("Content-Type", "application/json".to_string()),
            ("Authorization", format!("Bearer {}", api_key)),
        ])?;
        Ok(Self { request })
    }

    /// Send a chat request and get response stream (SSE).
    pub async fn chat_stream(&self, body: ChatRequest) -> Result<ByteStream> {
        let payload = serde_json::to_value(body)?;
        let r = self
            .request
            .post_stream("/v1/chat/completions", &payload, None)
            .await?;
        Ok(r)
    }

    /// Send a chat request and get full response.
    pub async fn chat_once(&self, body: ChatRequest) -> Result<String> {
        let payload = serde_json::to_value(body)?;
        let response = self
            .request
            .post("/v1/chat/completions", &payload, None)
            .await?;
        let r = response.text().await?;
        Ok(r)
    }
}
