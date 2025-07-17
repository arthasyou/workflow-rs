use core::str;
use std::sync::Arc;

use flow_data::{FlowData, output::FlowOutput};
use model_gateway_rs::{
    clients::llm::LlmClient,
    model::llm::{ChatMessage, LlmInput, LlmOutput},
    sdk::openai::OpenAIClient,
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
#[serde(rename_all = "camelCase")]
struct LLMNodeConfig {
    #[serde(rename = "apiHost")]
    pub base_url: String,
    pub api_key: String,
    #[serde(rename = "modelName")]
    pub model: String,
    pub prompt: Option<String>,
    pub system_prompt: Option<String>,
    pub temperature: Option<f32>,
}

#[derive(Clone)]
pub struct LLMNode {
    base: NodeBase,

    /// 系统提示词（system prompt）
    system_prompt: Option<String>,

    prompt: Option<String>,

    /// 生成温度
    temperature: Option<f32>,

    model_client: Arc<dyn ModelClient<LlmInput, LlmOutput> + Send + Sync>,
}

impl LLMNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: LLMNodeConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for InputNode".into()))?;

        let inner = OpenAIClient::new(&config.api_key, &config.base_url, &config.model)?;
        let client = LlmClient::new(inner);

        Ok(Self {
            base: NodeBase::new(id, processor),
            system_prompt: config.system_prompt,
            temperature: config.temperature,
            model_client: Arc::new(client),
            prompt: config.prompt,
        })
    }
}

impl std::fmt::Debug for LLMNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LLMNode")
            .field("system_prompt", &self.system_prompt)
            .field("temperature", &self.temperature)
            .field("prompt", &self.prompt)
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

        let msg = data_payload_to_message(&input, &self.system_prompt, &self.prompt)?;
        let input = LlmInput {
            messages: msg,
            max_tokens: None,
        };
        let r = self.model_client.infer(input).await?;
        let content = r.get_content();
        let response = FlowData::from(content);

        Ok(response.into())
    }
}

fn data_payload_to_message(
    input: &FlowData,
    system_prompt: &Option<String>,
    _prompt: &Option<String>,
) -> Result<Vec<ChatMessage>> {
    let content = input.as_text()?;
    let system_prompt = system_prompt.as_deref().unwrap_or("");

    Ok(vec![
        ChatMessage::system(system_prompt),
        ChatMessage::user(content),
    ])
}
