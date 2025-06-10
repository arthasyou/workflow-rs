use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 节点输入结构（暂时与 `Value` 相同，可扩展）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInput {
    pub data: Value,
}

impl NodeInput {
    pub fn new(data: Value) -> Self {
        Self { data }
    }
}
