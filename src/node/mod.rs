pub mod builder;
pub mod config;
pub mod orchestration;

use std::{collections::HashMap, fmt::Debug};

pub use builder::NodeBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    error::Result,
    processor::{InputProc, OutputProc, PROCESSOR_REGISTRY},
};

pub trait Executable: Send + Sync + Debug {
    /// 获取 NodeBase 引用
    fn get_base(&self) -> &NodeBase;

    /// 输入处理逻辑，委托给 NodeBase
    fn process_input(&self, input: Value) -> Result<Value> {
        self.get_base().process_input(input)
    }

    /// 核心执行逻辑：各节点自行实现
    fn core_execute(&self, input: Value) -> Result<Value>;

    /// 输出处理逻辑，委托给 NodeBase
    fn process_output(&self, output: Value) -> Result<Value> {
        self.get_base().process_output(output)
    }

    /// 统一执行流程
    fn execute(&self, input: Value) -> Result<Value> {
        let processed_input = self.process_input(input)?;
        let output = self.core_execute(processed_input)?;
        self.process_output(output)
    }

    /// 克隆自身并返回 Box<dyn Executable>
    fn clone_box(&self) -> Box<dyn Executable>;
}

/// 实现 `Clone` Trait
impl Clone for Box<dyn Executable> {
    fn clone(&self) -> Box<dyn Executable> {
        self.clone_box()
    }
}

/// 节点状态表示
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl Default for NodeState {
    fn default() -> Self {
        NodeState::Pending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeBase {
    pub id: String,
    pub state: NodeState,                  // 节点状态
    pub metadata: HashMap<String, String>, // 元数据字段
    pub input_processor_name: Option<String>,
    pub output_processor_name: Option<String>,
}

impl Default for NodeBase {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            state: NodeState::Pending,
            metadata: HashMap::new(),
            input_processor_name: None,
            output_processor_name: None,
        }
    }
}

impl NodeBase {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            input_processor_name: None,
            output_processor_name: None,
            state: NodeState::Pending,
            metadata: HashMap::new(),
        }
    }

    /// 更新状态
    pub fn set_state(&mut self, state: NodeState) {
        self.state = state;
    }

    /// 设置元数据
    pub fn set_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    /// 获取元数据
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    // 设置 Processor 名称（而不是实例）
    pub fn set_input_processor_name(&mut self, name: &str) {
        self.input_processor_name = Some(name.to_string());
    }

    pub fn set_output_processor_name(&mut self, name: &str) {
        self.output_processor_name = Some(name.to_string());
    }

    /// 动态获取输入处理器
    pub fn get_input_processor(&self) -> InputProc {
        if let Some(name) = &self.input_processor_name {
            PROCESSOR_REGISTRY.get_input(name)
        } else {
            None
        }
    }

    /// 动态获取输出处理器
    pub fn get_output_processor(&self) -> OutputProc {
        if let Some(name) = &self.output_processor_name {
            PROCESSOR_REGISTRY.get_output(name)
        } else {
            None
        }
    }
}

impl NodeBase {
    pub fn process_input(&self, input: Value) -> Result<Value> {
        if let Some(name) = &self.input_processor_name {
            if let Some(processor) = PROCESSOR_REGISTRY.get_input(name) {
                return processor.process(&self.id, &input, None);
            }
        }
        Ok(input)
    }

    pub fn process_output(&self, output: Value) -> Result<Value> {
        if let Some(name) = &self.output_processor_name {
            if let Some(processor) = PROCESSOR_REGISTRY.get_output(name) {
                return processor.process(&self.id, &output, None);
            }
        }
        Ok(output)
    }
}
