use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Data(DataNode),
    Control(ControlNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataNode {
    Prompt,
    Identity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlNode {
    Branch,
    Parallel,
    Repeat,
    Aggregator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessorMapping {
    pub input: Option<String>,
    pub output: Option<String>,
}

impl Default for DataProcessorMapping {
    fn default() -> Self {
        DataProcessorMapping {
            input: None,
            output: None,
        }
    }
}

/// 用于序列化和持久化的 Node 数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub node_type: NodeType,
    pub data: Value,
    pub processors: DataProcessorMapping,

    /// 输入数据 ID，可选
    pub input_id: Option<String>,

    pub output_id: Option<String>,
}

impl Node {
    pub fn new(
        id: &str,
        node_type: NodeType,
        data: Value,
        processors: DataProcessorMapping,
        input_id: Option<String>,
        output_id: Option<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            node_type,
            data,
            processors,
            input_id,
            output_id,
        }
    }

    pub fn new_start() -> Self {
        Node::new(
            "start",
            NodeType::Data(DataNode::Identity),
            Value::Null,
            DataProcessorMapping::default(),
            None,
            None,
        )
    }

    pub fn new_end() -> Self {
        Node::new(
            "end",
            NodeType::Control(ControlNode::Aggregator),
            Value::Null,
            DataProcessorMapping::default(),
            None,
            None,
        )
    }

    /// 判断节点是否为控制节点 (Orchestration Node)
    pub fn is_control_node(&self) -> bool {
        matches!(self.node_type, NodeType::Control(_))
    }
}
