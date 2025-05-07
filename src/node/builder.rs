use serde_json::Value;

use super::{InputProc, OutputProc};
use crate::node::{Node, NodeKind};

pub struct NodeBuilder {
    id: String,
    kind: NodeKind,
    config: Option<Value>,
    input_processor: InputProc<Value>,
    output_processor: OutputProc<Value>,
}

impl NodeBuilder {
    /// 创建新的 NodeBuilder
    pub fn new(id: &str, kind: NodeKind) -> Self {
        Self {
            id: id.to_string(),
            kind,
            config: None,
            input_processor: None,
            output_processor: None,
        }
    }

    /// 设置配置
    pub fn with_config(mut self, config: Value) -> Self {
        self.config = Some(config);
        self
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

    /// 构建 Node
    pub fn build(self) -> Node {
        Node {
            id: self.id,
            kind: self.kind,
            config: self.config,
            input_processor: self.input_processor,
            output_processor: self.output_processor,
        }
    }
}
