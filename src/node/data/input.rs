use std::sync::Arc;

use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{DataPayload, OutputData, context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase, config::InputConfig},
};

/// IdentityNode 节点：输入即输出，无处理逻辑
#[derive(Debug, Clone)]
pub struct InputNode {
    base: NodeBase,
    input: DataPayload,
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
        _input: Option<DataPayload>,
        _context: Arc<Context>,
    ) -> Result<OutputData> {
        Ok(OutputData::new_data(self.input.clone()))
    }
}
