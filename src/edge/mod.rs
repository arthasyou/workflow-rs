use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeType {
    Data,    // 数据传输路径
    Control, // 控制流路径（控制节点出口）
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    // /// Edge ID，用于唯一标识此 Edge
    pub id: String,

    /// 边类型：数据传输路径或控制流路径
    pub edge_type: EdgeType,

    /// 起始节点 ID（输出节点）
    pub source: String,

    /// 结束节点 ID（输入节点）
    pub target: String,

    /// 源节点句柄（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_handle: Option<String>,

    /// 目标节点句柄（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_handle: Option<String>,
}
