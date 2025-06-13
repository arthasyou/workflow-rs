use std::sync::Arc;

use flow_data::{FlowData, output::FlowOutput};
use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase, config::PromptConfig},
};

/// PromptNode 节点，用于接收输入并返回处理后的输出
#[derive(Debug, Clone)]
pub struct PromptNode {
    base: NodeBase,
    template: String,
}

impl PromptNode {
    /// 构建 PromptNode 实例，强制要求 `template` 字段必须存在
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: PromptConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for PromptNode".into()))?;

        // 校验 template 是否存在且非空
        if config.template.trim().is_empty() {
            return Err(Error::ExecutionError(
                "`template` field is required for PromptNode".into(),
            ));
        }

        Ok(Self {
            base: NodeBase::new(id, processor),
            template: config.template,
        })
    }
}

#[impl_executable]
impl Executable for PromptNode {
    async fn core_execute(
        &self,
        _input: Option<FlowData>,
        _context: Arc<Context>,
    ) -> Result<FlowOutput> {
        let data = FlowData::new_text(&self.template);
        Ok(data.into())
    }
}
