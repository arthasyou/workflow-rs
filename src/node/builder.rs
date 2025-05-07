use serde_json::Value;

use super::{InputProc, Node, NodeKind, OutputProc};
use crate::node::config::*;

pub struct NodeBuilder {
    id: String,
    kind: NodeKind,
    input_processor: InputProc<Value>,
    output_processor: OutputProc<Value>,
    config: Option<Value>,
}

impl NodeBuilder {
    /// 创建新的 NodeBuilder
    pub fn new(id: &str, kind: NodeKind) -> Self {
        Self {
            id: id.to_string(),
            kind,
            input_processor: None,
            output_processor: None,
            config: None,
        }
    }

    /// 设置 InputProcessor
    pub fn with_input_processor(mut self, processor: InputProc<Value>) -> Self {
        self.input_processor = processor;
        self
    }

    /// 设置 OutputProcessor
    pub fn with_output_processor(mut self, processor: OutputProc<Value>) -> Self {
        self.output_processor = processor;
        self
    }

    /// 设置 Prompt 节点配置
    pub fn with_prompt_config(mut self, config: PromptConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self.kind = NodeKind::Prompt;
        self
    }

    /// 设置 Model 节点配置
    pub fn with_model_config(mut self, config: ModelConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self.kind = NodeKind::Model;
        self
    }

    /// 设置 Branch 节点配置
    pub fn with_branch_config(mut self, config: BranchConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self.kind = NodeKind::Branch;
        self
    }

    /// 设置 Aggregator 节点配置
    pub fn with_aggregator_config(mut self, config: AggregatorConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self.kind = NodeKind::Aggregator;
        self
    }

    /// 设置 Transformer 节点配置
    pub fn with_transformer_config(mut self, config: TransformerConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self.kind = NodeKind::Transformer;
        self
    }

    /// 构建 Node
    pub fn build(self) -> Result<Node, String> {
        if self.config.is_none() {
            return Err("Configuration is missing".to_string());
        }

        Ok(Node {
            id: self.id,
            kind: self.kind,
            config: self.config,
            input_processor: self.input_processor,
            output_processor: self.output_processor,
        })
    }
}
