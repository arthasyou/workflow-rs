pub mod branch;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::{Error, Result},
    types::Executable,
};

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
            NodeKind::Branch => {
                if let Some(config) = &self.config {
                    if let Some(branches) = config.get("branches").and_then(|v| v.as_object()) {
                        if let Some(input_str) = input.as_str() {
                            if let Some(target_node) = branches.get(input_str) {
                                if let Some(target_str) = target_node.as_str() {
                                    return Ok(Value::String(target_str.to_string()));
                                }
                            }
                        }
                    }
                    if let Some(default) = config.get("default").and_then(|v| v.as_str()) {
                        return Ok(Value::String(default.to_string()));
                    }
                }
                Err(Error::BranchConfigMissing)
            }
        }
    }
}
