use std::{collections::HashMap, process::Output, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use workflow_macro::impl_executable;

use crate::{
    error::{Error, Result},
    model::{DataPayload, OutputData, context::Context, input, node::DataProcessorMapping, output},
    node::{Executable, NodeBase, config::BranchConfig},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchNode {
    pub base: NodeBase,
    pub branches: HashMap<String, String>,
    pub default: Option<String>,
}

impl BranchNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: BranchConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for BranchNode".into()))?;

        Ok(Self {
            base: NodeBase::new(id, processor),
            branches: config.branches,
            default: config.default,
        })
    }
}

#[impl_executable]
impl Executable for BranchNode {
    async fn core_execute(&self, input: DataPayload, _context: Arc<Context>) -> Result<OutputData> {
        if let Some(t) = input.as_text() {
            // 根据 input_str 找到下一个节点 ID
            let next_node_id = if let Some(target) = self.branches.get(t) {
                target.clone()
            } else if let Some(default) = &self.default {
                default.clone()
            } else {
                return Err(Error::NodeConfigMissing);
            };

            let output = OutputData::new_control(&next_node_id);

            Ok(output)
        } else {
            return Err(Error::InvalidBranchInput);
        }
    }
}
