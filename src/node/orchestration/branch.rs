use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use workflow_macro::impl_executable;

use crate::{
    error::{Error, Result},
    model::NodeOutput,
    node::{Executable, NodeBase},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchNode {
    pub base: NodeBase,
    pub branches: HashMap<String, String>,
    pub default: Option<String>,
}

impl BranchNode {
    pub fn new(id: &str, branches: HashMap<String, String>, default: Option<String>) -> Self {
        let base = NodeBase::new(id);
        Self {
            base,
            branches,
            default,
        }
    }
}

#[impl_executable]
impl Executable for BranchNode {
    async fn core_execute(&self, input: Value) -> Result<Value> {
        let input_str = input.as_str().ok_or(Error::InvalidBranchInput)?;

        let next_node_id = if let Some(target) = self.branches.get(input_str) {
            target.clone()
        } else if let Some(default) = &self.default {
            default.clone()
        } else {
            return Err(Error::BranchConfigMissing);
        };

        Ok(serde_json::to_value(NodeOutput::new(&next_node_id, input))?)
    }
}
