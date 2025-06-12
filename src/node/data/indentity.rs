use std::sync::Arc;

use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{DataPayload, OutputData, context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase},
};

/// IdentityNode 节点：输入即输出，无处理逻辑
#[derive(Debug, Clone)]
pub struct IdentityNode {
    base: NodeBase,
}

impl IdentityNode {
    pub fn new(id: &str, _data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        Ok(Self {
            base: NodeBase::new(id, processor),
        })
    }
}

#[impl_executable]
impl Executable for IdentityNode {
    async fn core_execute(
        &self,
        input: Option<DataPayload>,
        _context: Arc<Context>,
    ) -> Result<OutputData> {
        match input {
            Some(data) => Ok(OutputData::new_data(data)),
            None => Err(Error::ExecutionError("No input data provided".into())),
        }
    }
}
