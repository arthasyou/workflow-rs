use super::core::ModelClient;
use crate::types::text::{TextPrompt, TextResponse};

#[async_trait::async_trait]
pub trait TextGeneration: ModelClient<Input = TextPrompt, Output = TextResponse> {}
