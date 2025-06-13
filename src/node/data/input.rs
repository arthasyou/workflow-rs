use std::sync::Arc;

use flow_data::{FlowData, output::FlowOutput};
use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase, config::InputConfig},
};

/// IdentityNode 节点：输入即输出，无处理逻辑
#[derive(Debug, Clone)]
pub struct InputNode {
    base: NodeBase,
    input: FlowData,
}

impl InputNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: InputConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for InputNode".into()))?;

        Ok(Self {
            base: NodeBase::new(id, processor),
            input: config.input.into(),
        })
    }
}

#[impl_executable]
impl Executable for InputNode {
    async fn core_execute(
        &self,
        _input: Option<FlowData>,
        _context: Arc<Context>,
    ) -> Result<FlowOutput> {
        Ok(self.input.clone().into())
    }
}
