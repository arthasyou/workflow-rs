use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPrompt {
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    // 可扩展：stop, max_tokens 等
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextResponse {
    pub content: String,
    pub raw: Option<serde_json::Value>,
}
