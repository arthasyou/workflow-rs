use std::sync::Arc;

use serde_json::Value;
use workflow_macro::impl_executable;

use crate::{
    error::{Error, Result},
    model::{DataPayload, OutputData, context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase, config::RepeatConfig},
};

#[derive(Debug, Clone)]
pub struct RepeatNode {
    pub base: NodeBase,
    pub child_id: String,
    pub max_iterations: usize,
}

impl RepeatNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: RepeatConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for RepeatNode".into()))?;

        Ok(Self {
            base: NodeBase::new(id, processor),
            child_id: config.child_id,
            max_iterations: config.max_iterations,
        })
    }
}

#[impl_executable]
impl Executable for RepeatNode {
    async fn core_execute(&self, input: DataPayload, context: Arc<Context>) -> Result<OutputData> {
        let mut current_input = input;

        for _ in 0 .. self.max_iterations {
            let child_node = context
                .get_node(&self.child_id)
                .ok_or_else(|| Error::NodeNotFound(self.child_id.clone()))?;

            let output = child_node
                .execute(current_input.clone(), context.clone())
                .await?;

            if let OutputData::Data(data) = output {
                current_input = data;
            } else {
                return Err(Error::ExecutionError(
                    "Invalid output from child node".into(),
                ));
            }
        }

        Ok(OutputData::Data(current_input))
    }
}
