use serde::{Deserialize, Serialize};

use super::DataPayload;

/// 节点执行输出结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputData {
    Control(String),
    Data(DataPayload),
    Parallel(Vec<NodeOutput>),
}

impl OutputData {
    /// 创建新的控制信号输出
    pub fn new_control(control: &str) -> Self {
        Self::Control(control.to_string())
    }

    /// 创建新的数据输出
    pub fn new_data(data: DataPayload) -> Self {
        Self::Data(data)
    }

    pub fn default_parallel() -> Self {
        Self::Parallel(Vec::new())
    }

    pub fn insert_parallel(&mut self, node_output: NodeOutput) {
        if let Self::Parallel(outputs) = self {
            outputs.push(node_output);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeOutput {
    pub next_node_id: String,
    pub data: OutputData,
}

impl NodeOutput {
    /// 创建新的 NodeOutput
    pub fn new(next_node_id: &str, data: OutputData) -> Self {
        Self {
            next_node_id: next_node_id.to_string(),
            data,
        }
    }
}
