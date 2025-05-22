use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use workflow_macro::impl_executable;

use crate::{
    error::Result,
    model::{DataPayload, OutputData, context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase},
};

/// IdentityNode 节点：输入即输出，无处理逻辑
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    async fn core_execute(&self, input: DataPayload, _context: Arc<Context>) -> Result<OutputData> {
        Ok(OutputData::new_data(input))
    }
}
