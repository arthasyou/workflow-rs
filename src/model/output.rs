use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 节点执行输出结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeOutput {
    pub next_node_id: String,
    pub input: Value,
}

impl NodeOutput {
    /// 创建新的 NodeOutput
    pub fn new(next_node_id: &str, input: Value) -> Self {
        Self {
            next_node_id: next_node_id.to_string(),
            input,
        }
    }
}
