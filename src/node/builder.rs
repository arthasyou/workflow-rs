use std::sync::Arc;

use workflow_error::Result;

use super::{
    Executable,
    control::{AggregatorNode, BranchNode, ParallelNode, RepeatNode},
    data::{PromptNode, indentity::IdentityNode},
};
use crate::{
    model::node::{ControlNode, DataNode, Node, NodeType},
    node::data::{input::InputNode, llm::LLMNode},
};

/// 构建节点实例
/// 输入：`Node` 数据结构
/// 输出：运行时节点实例 `Arc<dyn Executable>`
pub fn build_node(node: &Node) -> Result<Arc<dyn Executable>> {
    let id = &node.id;
    let data = node.data.clone();
    let processors = &node.processors;

    let executable: Box<dyn Executable> = match &node.node_type {
        NodeType::Data(exec_node) => match exec_node {
            DataNode::Input => Box::new(InputNode::new(id, data, processors)?),
            DataNode::Prompt => Box::new(PromptNode::new(id, data, processors)?),
            DataNode::Identity => Box::new(IdentityNode::new(id, data, processors)?),
            DataNode::LLM => Box::new(LLMNode::new(id, data, processors)?),
        },
        NodeType::Control(orch_node) => match orch_node {
            ControlNode::Branch => Box::new(BranchNode::new(id, data, processors)?),
            ControlNode::Parallel => Box::new(ParallelNode::new(id, data, processors)?),
            ControlNode::Repeat => Box::new(RepeatNode::new(id, data, processors)?),
            ControlNode::Aggregator => Box::new(AggregatorNode::new(id, data, processors)?),
        },
    };

    Ok(Arc::from(executable))
}
