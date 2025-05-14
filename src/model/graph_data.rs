use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::node::Node;
use crate::edge::Edge;

/// 持久化使用的 Graph 数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    /// 节点数据：节点 ID -> 节点配置
    pub nodes: HashMap<String, Node>,
    /// 边信息
    pub edges: Vec<Edge>,
}

impl GraphData {
    /// 创建新的 GraphData
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// 添加节点数据
    pub fn add_node(&mut self, id: &str, data: Node) {
        self.nodes.insert(id.to_string(), data);
    }

    /// 添加边信息
    pub fn add_edge(&mut self, start: &str, end: &str) {
        self.edges.push(Edge {
            start: start.to_string(),
            end: end.to_string(),
        });
    }
}
