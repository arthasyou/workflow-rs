pub mod branch;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use self::branch::execute_branch;
use crate::{error::Result, types::Executable};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeKind {
    Prompt,
    Model,
    Retriever,
    Branch,
    // 以后可以加更多类型
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
    pub config: Option<Value>, // 新增字段
}

impl Executable for Node {
    fn execute(&self, input: Value) -> Result<Value> {
        match self.kind {
            NodeKind::Prompt => {
                if let Some(cfg) = &self.config {
                    if let Some(tpl) = cfg.get("template").and_then(|v| v.as_str()) {
                        let filled = tpl.replace("{input}", input.as_str().unwrap_or(""));
                        return Ok(Value::String(filled));
                    }
                }
                Ok(Value::String(format!("Prompted: {}", input)))
            }
            NodeKind::Model => Ok(Value::String(format!("Model output based on: {}", input))),
            NodeKind::Retriever => Ok(Value::String(format!("Retrieved info for: {}", input))),
            NodeKind::Branch => execute_branch(&self.config, &input),
        }
    }
}
