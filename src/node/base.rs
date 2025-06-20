use std::collections::HashMap;

use flow_data::{FlowData, output::FlowOutput};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    model::node::DataProcessorMapping,
    processor::{InputProcessor, OutputProcessor, PROCESSOR_REGISTRY},
};

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

#[derive(Debug, Clone)]
pub struct NodeBase {
    pub id: String,
    pub state: NodeState,
    pub metadata: HashMap<String, String>,
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
    pub async fn process_input(&self, input: Option<FlowData>) -> Option<FlowData> {
        if let Some(name) = &self.input_processor_name {
            if let Some(processor) = PROCESSOR_REGISTRY.get_input(name) {
                match input {
                    Some(data) => {
                        // 如果输入数据存在，使用处理器处理
                        return processor.process(data).await;
                    }
                    None => {
                        // 如果没有输入数据，直接返回 None
                        return None;
                    }
                }
            }
        }
        input
    }

    pub async fn process_output(&self, output: FlowOutput) -> Option<FlowOutput> {
        if let Some(name) = &self.output_processor_name {
            if let Some(processor) = PROCESSOR_REGISTRY.get_output(name) {
                return processor.process(output).await;
            }
        }
        Some(output)
    }
}

impl NodeBase {
    pub fn new(id: &str, processor: &DataProcessorMapping) -> Self {
        Self {
            id: id.to_string(),
            input_processor_name: processor.input.to_owned(),
            output_processor_name: processor.output.to_owned(),
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
    pub fn get_input_processor(&self) -> InputProcessor {
        if let Some(name) = &self.input_processor_name {
            PROCESSOR_REGISTRY.get_input(name)
        } else {
            None
        }
    }

    /// 动态获取输出处理器
    pub fn get_output_processor(&self) -> OutputProcessor {
        if let Some(name) = &self.output_processor_name {
            PROCESSOR_REGISTRY.get_output(name)
        } else {
            None
        }
    }
}
