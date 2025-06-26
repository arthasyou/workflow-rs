use core::str;
use std::sync::Arc;

use flow_data::{FlowData, output::FlowOutput};
use model_client::{
    clients::openai::OpenAITextClient,
    sdk::openai::{ChatMessage, ChatResponse},
    traits::ModelClient,
};
use serde::Deserialize;
use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase},
};

#[derive(Debug, Clone, Deserialize)]
struct LLMNodeConfig {
    base_url: String,
    api_key: String,
    model: String,
    system_prompt: Option<String>,
    temperature: Option<f32>,
    top_p: Option<f32>,
}

#[derive(Clone)]
pub struct LLMNode {
    base: NodeBase,

    /// 系统提示词（system prompt）
    system_prompt: Option<String>,

    /// 生成温度
    temperature: Option<f32>,

    /// Top-p 采样
    top_p: Option<f32>,

    model_client:
        Arc<dyn ModelClient<Input = Vec<ChatMessage>, Output = ChatResponse> + Send + Sync>,
}

impl LLMNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: LLMNodeConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for InputNode".into()))?;
        let client = OpenAITextClient::new(&config.api_key, &config.base_url, &config.model)?;

        Ok(Self {
            base: NodeBase::new(id, processor),
            system_prompt: config.system_prompt,
            temperature: config.temperature,
            top_p: config.top_p,
            model_client: Arc::new(client),
        })
    }
}

impl std::fmt::Debug for LLMNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LLMNode")
            .field("system_prompt", &self.system_prompt)
            .field("temperature", &self.temperature)
            .field("top_p", &self.top_p)
            .finish()
    }
}

#[impl_executable]
impl Executable for LLMNode {
    async fn core_execute(
        &self,
        input: Option<FlowData>,
        _context: Arc<Context>,
    ) -> Result<FlowOutput> {
        let input = match input {
            Some(data) => data,
            None => {
                return Err(Error::ExecutionError("LLMNode requires input data".into()));
            }
        };

        let msg = data_payload_to_message(&input)?;
        let r = self.model_client.infer(msg).await?;
        let response = match r.first_message() {
            Some(content) => FlowData::from(content),
            None => {
                return Err(Error::ExecutionError(
                    "LLMNode received empty response".into(),
                ));
            }
        };

        Ok(response.into())
    }
}

fn data_payload_to_message(input: &FlowData) -> Result<Vec<ChatMessage>> {
    let prompt = input.as_text()?;
    Ok(vec![ChatMessage::user(prompt)])
}
