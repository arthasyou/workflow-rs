use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
    pub config: Option<Value>, // 新增字段
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeKind {
    Prompt,
    Model,
    Retriever,
    // 以后可以加更多类型
}
