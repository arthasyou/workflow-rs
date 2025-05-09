use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Executable(ExecutableNode),
    Orchestration(OrchestrationNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutableNode {
    Prompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrchestrationNode {
    Branch,
    Parallel,
    Repeat,
}

/// 用于序列化和持久化的 Node 数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub node_type: NodeType,
    pub data: Value,
}

impl Node {
    pub fn new(id: &str, node_type: NodeType, data: Value) -> Self {
        Self {
            id: id.to_string(),
            node_type,
            data,
        }
    }
}
