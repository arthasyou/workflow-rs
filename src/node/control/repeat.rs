use std::sync::Arc;

use flow_data::{FlowData, output::FlowOutput};
use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{context::Context, node::DataProcessorMapping},
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
    async fn core_execute(
        &self,
        input: Option<FlowData>,
        context: Arc<Context>,
    ) -> Result<FlowOutput> {
        match input {
            Some(data) => {
                let mut current_input = data;
                for _ in 0 .. self.max_iterations {
                    let child_node = context
                        .get_node(&self.child_id)
                        .ok_or_else(|| Error::NodeNotFound(self.child_id.clone().into()))?;

                    let output = child_node
                        .execute(Some(current_input), context.clone())
                        .await?;
                    current_input = output.into_data()?;
                }

                Ok(current_input.into())
            }
            None => return Err(Error::ExecutionError("No input data provided".into())),
        }
    }
}
