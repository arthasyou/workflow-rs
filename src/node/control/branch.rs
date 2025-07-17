use std::sync::Arc;

use flow_data::{FlowData, output::FlowOutput};
use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{context::Context, node::DataProcessorMapping},
    node::{
        Executable, NodeBase,
        config::{BranchConfig, BranchPayload},
    },
};

#[derive(Debug, Clone)]
pub struct BranchNode {
    pub base: NodeBase,
    pub branches: Vec<BranchPayload>,
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
    async fn core_execute(
        &self,
        input: Option<FlowData>,
        _context: Arc<Context>,
    ) -> Result<FlowOutput> {
        match input {
            None => Err(Error::ExecutionError("No input data provided".into())),
            Some(data) => {
                let input_str = data.as_text()?;
                let node_id = match_branch(input_str, &self.branches);
                let next_node_id = match node_id {
                    None => "default",
                    Some(id) => id,
                };

                // println!("Next node ID: {}", next_node_id);

                Ok((next_node_id, data).into())
            }
        }
    }
}

fn match_branch<'a>(input: &str, branches: &'a [BranchPayload]) -> Option<&'a str> {
    // println!("Matching input: {}", input);
    // println!("Against branches: {:?}", branches);

    for branch in branches {
        let matched = match branch.value_type.as_str() {
            "string" => match branch.condition.as_str() {
                "==" => input == branch.value,
                "!=" => input != branch.value,
                "contains" => input.contains(&branch.value),
                _ => false,
            },
            "number" => {
                let input_num = input.parse::<f64>().ok()?;
                let target = branch.value.parse::<f64>().ok()?;
                match branch.condition.as_str() {
                    "==" => input_num == target,
                    "!=" => input_num != target,
                    ">" => input_num > target,
                    ">=" => input_num >= target,
                    "<" => input_num < target,
                    "<=" => input_num <= target,
                    _ => false,
                }
            }
            _ => false,
        };

        if matched {
            return Some(branch.id.as_str());
        }
    }

    None
}
