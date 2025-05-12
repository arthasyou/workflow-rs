use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use workflow_macro::impl_executable;

use crate::{
    error::{Error, Result},
    model::{NodeOutput, context::Context},
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
    async fn core_execute(&self, input: Value, context: &Context) -> Result<Value> {
        let input_str = input.as_str().ok_or(Error::InvalidBranchInput)?;

        // 根据 input_str 找到下一个节点 ID
        let next_node_id = if let Some(target) = self.branches.get(input_str) {
            target.clone()
        } else if let Some(default) = &self.default {
            default.clone()
        } else {
            return Err(Error::NodeConfigMissing);
        };

        // 在 context 中查找下一个节点实例
        let next_node = context
            .get_node(&next_node_id)
            .ok_or(Error::NodeNotFound(next_node_id.clone()))?;

        // 执行下一个节点
        let output = next_node.execute(input.clone(), context).await?;

        Ok(serde_json::to_value(NodeOutput::new(
            &next_node_id,
            output,
        ))?)
    }
}
