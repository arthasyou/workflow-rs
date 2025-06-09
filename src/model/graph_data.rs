use serde::{Deserialize, Serialize};

use super::node::Node;
use crate::{edge::Edge, graph::Graph};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeData {
    pub id: String,
    pub start: String,
    pub end: String,
}

/// 持久化使用的 Graph 数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    /// 节点数据：节点 ID -> 节点配置
    pub nodes: Vec<Node>,
    /// 边信息
    pub edges: Vec<EdgeData>,
    /// 起始节点
    pub start_node: Option<String>,
    /// 结束节点
    pub end_node: Option<String>,
}

impl From<Edge> for EdgeData {
    fn from(edge: Edge) -> Self {
        Self {
            id: edge.id,
            start: edge.start,
            end: edge.end,
        }
    }
}

impl From<Graph> for GraphData {
    fn from(graph: Graph) -> Self {
        Self {
            nodes: graph.nodes.values().cloned().collect(),
            edges: graph.edges.iter().cloned().map(EdgeData::from).collect(),
            start_node: graph.start_node,
            end_node: graph.end_node,
        }
    }
}

impl From<GraphData> for Graph {
    fn from(data: GraphData) -> Self {
        let mut graph = Graph::new();
        for node in data.nodes {
            graph.add_node(node).unwrap();
        }
        for edge in data.edges {
            graph.add_edge(&edge.start, &edge.end).unwrap();
        }
        graph.start_node = data.start_node;
        graph.end_node = data.end_node;
        graph
    }
}

impl GraphData {
    /// 创建新的 GraphData
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            start_node: None,
            end_node: None,
        }
    }

    /// 添加节点数据
    pub fn add_node(&mut self, data: Node) {
        self.nodes.push(data);
    }

    /// 添加边信息
    pub fn add_edge(&mut self, start: &str, end: &str) {
        self.edges.push(EdgeData {
            id: format!("{}-{}", start, end),
            start: start.to_string(),
            end: end.to_string(),
        });
    }
}
