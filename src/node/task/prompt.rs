use serde::{Deserialize, Serialize};
use serde_json::Value;
use workflow_macro::impl_executable;

use crate::{
    error::Result,
    node::{Executable, NodeBase},
};

/// PromptNode 节点，用于接收输入并返回处理后的输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptNode {
    base: NodeBase,
}

impl PromptNode {
    pub fn new(id: &str) -> Self {
        Self {
            base: NodeBase::new(id),
        }
    }
}

#[impl_executable]
impl Executable for PromptNode {
    /// 核心执行逻辑：直接返回输入内容，用于测试流程
    async fn core_execute(&self, input: Value) -> Result<Value> {
        // 将输入转为字符串并返回
        let input_str = input.to_string();
        Ok(Value::String(format!("PromptNode Output: {}", input_str)))
    }
}
