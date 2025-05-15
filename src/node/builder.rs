use std::sync::Arc;

use super::{Executable, data::PromptNode};
use crate::{
    error::Result,
    model::node::{ControlNode, DataNode, Node, NodeType},
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
            DataNode::Prompt => Box::new(PromptNode::new(id, data, processors)?),
        },
        NodeType::Control(orch_node) => match orch_node {
            ControlNode::Branch => {
                todo!("BranchNode not implemented")
                // Box::new(BranchNode::new(id, data, processors)?)
            }
            ControlNode::Parallel => {
                todo!("ParallelNode not implemented")
            }
            ControlNode::Repeat => {
                todo!("RepeatNode not implemented")
            }
        },
    };

    Ok(Arc::from(executable))
}
