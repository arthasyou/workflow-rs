use std::{collections::HashMap, sync::Arc};

use flow_data::{FlowData, output::FlowOutput};
use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase, config::BranchConfig},
};

#[derive(Debug, Clone)]
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
            branches: config.to_hashmap(),
            default: config.default,
        })
    }
}

#[impl_executable]
impl Executable for BranchNode {
    async fn core_execute(
        &self,
        input: Option<FlowData>,
        _context: Arc<Context>,
    ) -> Result<FlowOutput> {
        match input {
            None => Err(Error::ExecutionError("No input data provided".into())),
            Some(data) => {
                // 根据 input_str 找到下一个节点 ID
                let next_node_id = if let Some(target) = self.branches.get(data.as_text().unwrap())
                {
                    target.clone()
                } else if let Some(default) = &self.default {
                    default.clone()
                } else {
                    return Err(Error::NodeConfigMissing);
                };

                Ok((next_node_id, data).into())
            }
        }
    }
}
