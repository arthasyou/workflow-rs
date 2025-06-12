use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{
        OutputData,
        context::Context,
        data_payload::{DataPayload, SingleData},
        node::DataProcessorMapping,
    },
    node::{Executable, NodeBase},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMNode {
    base: NodeBase,

    /// 模型名称，如 "gpt-3.5-turbo"、"qwen-plus"
    model: String,

    /// LLM 接口的 base URL
    api_base: String,

    /// API Key，用于认证（可选）
    api_key: Option<String>,

    /// 系统提示词（system prompt）
    system_prompt: Option<String>,

    /// 生成温度
    temperature: Option<f32>,

    /// Top-p 采样
    top_p: Option<f32>,
}

impl LLMNode {
    pub fn new(id: &str, _data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        Ok(Self {
            base: NodeBase::new(id, processor),
            model: "gpt-3.5-turbo".to_string(),
            api_base: "".to_string(),
            api_key: None,
            system_prompt: None,
            temperature: None,
            top_p: None,
        })
    }
}

#[impl_executable]
impl Executable for LLMNode {
    async fn core_execute(
        &self,
        input: Option<DataPayload>,
        _context: Arc<Context>,
    ) -> Result<OutputData> {
        let input = match input {
            Some(data) => data,
            None => {
                return Err(Error::ExecutionError("LLMNode requires input data".into()));
            }
        };

        // TODO: 实际 LLM 调用（此处留作占位）
        let response = call_llm_model(&input).await?;

        Ok(OutputData::new_data(response))
    }
}

/// 模拟或集成实际 LLM 请求的函数
async fn call_llm_model(input: &DataPayload) -> Result<DataPayload> {
    // TODO: 根据 input.prompt 调用实际模型，例如 OpenAI/Qwen
    // let prompt = match input {
    //     DataPayload::Single { value, .. } => value.clone(),
    //     _ => {
    //         return Err(Error::ExecutionError(
    //             "Unsupported input format for LLMNode".into(),
    //         ));
    //     }
    // };

    let prompt = "prompt";
    let simulated = format!("LLM回答：{}", prompt);
    Ok(DataPayload::new_single(SingleData::new_text(&simulated)))
}
