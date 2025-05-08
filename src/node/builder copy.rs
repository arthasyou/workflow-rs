use serde_json::Value;

use super::NodeBase;
use crate::{
    node::config::*,
    processor::{InputProc, OutputProc},
};

pub struct NodeBuilder {
    id: String,
    input_processor: InputProc,
    output_processor: OutputProc,
    config: Option<Value>,
}

impl NodeBuilder {
    /// 创建新的 NodeBuilder
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            input_processor: None,
            output_processor: None,
            config: None,
        }
    }

    /// 设置 InputProcessor
    pub fn with_input_processor(mut self, processor: InputProc) -> Self {
        self.input_processor = processor;
        self
    }

    /// 设置 OutputProcessor
    pub fn with_output_processor(mut self, processor: OutputProc) -> Self {
        self.output_processor = processor;
        self
    }

    /// 设置 Prompt 节点配置
    pub fn with_prompt_config(mut self, config: PromptConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self
    }

    /// 设置 Model 节点配置
    pub fn with_model_config(mut self, config: ModelConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self
    }

    /// 设置 Branch 节点配置
    pub fn with_branch_config(mut self, config: BranchConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self
    }

    /// 设置 Aggregator 节点配置
    pub fn with_aggregator_config(mut self, config: AggregatorConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self
    }

    /// 设置 Transformer 节点配置
    pub fn with_transformer_config(mut self, config: TransformerConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());

        self
    }

    // pub fn build(self) -> Result<NodeBase, String> {
    //     if self.config.is_none() {
    //         return Err("Configuration is missing".to_string());
    //     }

    //     Ok(NodeBase {
    //         id: self.id,
    //         config: self.config,
    //         input_processor: self.input_processor,
    //         output_processor: self.output_processor,
    //     })
    // }
}
