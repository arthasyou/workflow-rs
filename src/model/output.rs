use serde::{Deserialize, Serialize};

use super::DataPayload;

/// 节点执行输出结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputData {
    Control(String),
    Data(DataPayload),
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
}

pub struct NodeOutput {
    pub next_node_id: String,
    pub input: DataPayload,
}

impl NodeOutput {
    /// 创建新的 NodeOutput
    pub fn new(next_node_id: &str, input: DataPayload) -> Self {
        Self {
            next_node_id: next_node_id.to_string(),
            input,
        }
    }
}
