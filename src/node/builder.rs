use std::{marker::PhantomData, sync::Arc};

use serde_json::Value;

use super::{Executable, NodeBase, orchestration::branch::BranchNode, task::prompt::PromptNode};
use crate::{
    error::{Error, Result},
    model::node::{ExecutableNode, Node, NodeType, OrchestrationNode},
    node::config::*,
    processor::{InputProcessor, OutputProcessor},
};

/// 泛型节点构建器，支持任意节点类型
pub struct NodeBuilder<T> {
    id: String,
    input_processor: InputProcessor,
    output_processor: OutputProcessor,
    config: Option<Value>,
    _marker: PhantomData<T>, // 用于指定节点类型
}

impl<T> NodeBuilder<T> {
    /// 创建新的 NodeBuilder
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            input_processor: None,
            output_processor: None,
            config: None,
            _marker: PhantomData,
        }
    }

    /// 设置输入处理器
    pub fn with_input_processor(mut self, processor: InputProcessor) -> Self {
        self.input_processor = processor;
        self
    }

    /// 设置输出处理器
    pub fn with_output_processor(mut self, processor: OutputProcessor) -> Self {
        self.output_processor = processor;
        self
    }

    /// 设置任意节点配置（通用）
    pub fn with_config(mut self, config: Value) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置特定节点配置（Prompt）
    pub fn with_prompt_config(mut self, config: PromptConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self
    }

    /// 设置特定节点配置（Model）
    pub fn with_model_config(mut self, config: ModelConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self
    }

    /// 设置特定节点配置（Branch）
    pub fn with_branch_config(mut self, config: BranchConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self
    }

    /// 设置特定节点配置（Aggregator）
    pub fn with_aggregator_config(mut self, config: AggregatorConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self
    }

    /// 设置特定节点配置（Transformer）
    pub fn with_transformer_config(mut self, config: TransformerConfig) -> Self {
        self.config = Some(serde_json::to_value(config).unwrap());
        self
    }

    /// 构建节点
    pub fn build(self) -> Result<NodeBase> {
        if self.config.is_none() {
            return Err(Error::NodeConfigMissing);
        }

        Ok(NodeBase {
            id: self.id,
            state: Default::default(),
            metadata: Default::default(),
            input_processor_name: None,
            output_processor_name: None,
        })
    }
}

/// 构建节点实例
/// 输入：`Node` 数据结构
/// 输出：运行时节点实例 `Arc<dyn Executable>`
pub fn build_node(node: &Node) -> Result<Arc<dyn Executable>> {
    let id = &node.id;
    let data = node.data.clone();

    match &node.node_type {
        NodeType::Executable(exec_node) => match exec_node {
            ExecutableNode::Prompt => {
                // todo!("ParallelNode not implemented")
                let node = PromptNode::new(id, data)?;
                Ok(Arc::new(node))
            }
        },
        NodeType::Orchestration(orch_node) => match orch_node {
            OrchestrationNode::Branch => {
                todo!("ParallelNode not implemented")
                // let node = BranchNode::new(id, data);
                // Ok(Arc::new(node))
            }
            OrchestrationNode::Parallel => {
                todo!("ParallelNode not implemented")
            }
            OrchestrationNode::Repeat => {
                todo!("RepeatNode not implemented")
            }
        },
    }
}
