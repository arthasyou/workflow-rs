pub mod builder;
pub mod components;
pub mod config;

use std::fmt::{Debug, Formatter};

pub use builder::NodeBuilder;
use components::{execute_aggregator, execute_branch, execute_transformer};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::Result,
    processor::{InputProc, OutputProc},
    types::Executable,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeKind {
    Prompt,
    Model,
    Retriever,
    Branch,
    Aggregator, // 新增类型，用于数据聚合
    Transformer, /* 新增类型，用于数据转换
                 * 以后可以加更多类型 */
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
    pub config: Option<Value>,
    #[serde(skip)] // 不参与序列化
    pub input_processor: InputProc,
    #[serde(skip)] // 不参与序列化
    pub output_processor: OutputProc,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("config", &self.config)
            .finish() // 不打印 input_processor 和 output_processor
    }
}

impl Executable for Node {
    fn execute(&self, input: Value) -> Result<Value> {
        let mut processed_input = input.clone();

        // 执行 InputProcessor（如果存在）
        if let Some(input_proc) = &self.input_processor {
            processed_input = input_proc.process(&self.id, &processed_input, None)?;
        }

        // 执行节点核心逻辑
        let mut output = match self.kind {
            NodeKind::Prompt => {
                if let Some(cfg) = &self.config {
                    if let Some(tpl) = cfg.get("template").and_then(|v| v.as_str()) {
                        let filled = tpl.replace("{input}", processed_input.as_str().unwrap_or(""));
                        Value::String(filled)
                    } else {
                        Value::String(format!("Prompted: {}", processed_input))
                    }
                } else {
                    Value::String(format!("Prompted: {}", processed_input))
                }
            }
            NodeKind::Model => Value::String(format!("Model output based on: {}", processed_input)),
            NodeKind::Retriever => {
                Value::String(format!("Retrieved info for: {}", processed_input))
            }
            NodeKind::Branch => execute_branch(&self.config, &processed_input)?,
            NodeKind::Aggregator => execute_aggregator(&self.config, &processed_input)?,
            NodeKind::Transformer => execute_transformer(&self.config, &processed_input)?,
        };

        // 执行 OutputProcessor（如果存在）
        if let Some(output_proc) = &self.output_processor {
            output = output_proc.process(&self.id, &output, None)?;
        }

        Ok(output)
    }
}
