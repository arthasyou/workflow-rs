use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::node::Node;
use crate::edge::{Edge, EdgeType};

/// 持久化使用的 Graph 数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    /// 节点数据：节点 ID -> 节点配置
    pub nodes: HashMap<String, Node>,
    /// 边信息
    pub edges: Vec<Edge>,
    /// 起始节点
    pub start_node: Option<String>,
    /// 结束节点
    pub end_node: Option<String>,
}

impl GraphData {
    /// 创建新的 GraphData
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            start_node: None,
            end_node: None,
        }
    }

    /// 添加节点数据
    pub fn add_node(&mut self, id: &str, data: Node) {
        self.nodes.insert(id.to_string(), data);
    }

    /// 添加边信息
    pub fn add_edge(&mut self, start: &str, end: &str, edge_type: EdgeType) {
        self.edges.push(Edge {
            start: start.to_string(),
            end: end.to_string(),
            edge_type,
        });
    }
}
