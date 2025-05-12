use serde::{Deserialize, Serialize};
use serde_json::Value;
use workflow_macro::impl_executable;

use crate::{
    error::{Error, Result},
    model::context::Context,
    node::{Executable, NodeBase, config::PromptConfig},
};

/// PromptNode 节点，用于接收输入并返回处理后的输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptNode {
    base: NodeBase,
    template: String,
}

impl PromptNode {
    /// 构建 PromptNode 实例，强制要求 `template` 字段必须存在
    pub fn new(id: &str, data: Value) -> Result<Self> {
        let config: PromptConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for PromptNode".into()))?;

        // 校验 template 是否存在且非空
        if config.template.trim().is_empty() {
            return Err(Error::ExecutionError(
                "`template` field is required for PromptNode".into(),
            ));
        }

        Ok(Self {
            base: NodeBase::new(id),
            template: config.template,
        })
    }
}

#[impl_executable]
impl Executable for PromptNode {
    /// 核心执行逻辑
    async fn core_execute(&self, _input: Value, _context: &Context) -> Result<Value> {
        let response = &self.template;
        Ok(Value::String(response.to_string()))
    }
}
