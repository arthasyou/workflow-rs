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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessorMapping {
    pub input: Option<String>,
    pub output: Option<String>,
}

impl Default for DataProcessorMapping {
    fn default() -> Self {
        DataProcessorMapping {
            input: None,
            output: None,
        }
    }
}

/// 用于序列化和持久化的 Node 数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub node_type: NodeType,
    pub data: Value,
    pub processors: DataProcessorMapping,
}

impl Node {
    pub fn new(
        id: &str,
        node_type: NodeType,
        data: Value,
        processors: DataProcessorMapping,
    ) -> Self {
        Self {
            id: id.to_string(),
            node_type,
            data,
            processors,
        }
    }
}
