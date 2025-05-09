use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    edge::Edge,
    error::{Error, Result},
    model::Context,
    node::Executable,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Graph {
    /// 节点存储：Key 为节点 ID，Value 为可执行节点
    #[serde(skip)]
    pub nodes: HashMap<String, Arc<dyn Executable>>,
    pub edges: Vec<Edge>,
    pub compiled: bool,

    /// 前置节点与后继节点映射
    pub predecessors: HashMap<String, HashSet<String>>,
    pub successors: HashMap<String, HashSet<String>>,
}

impl Graph {
    /// 创建新的图结构
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            compiled: false,
            predecessors: HashMap::new(),
            successors: HashMap::new(),
        }
    }

    /// 添加节点（节点类型需实现 Executable）
    pub fn add_node<T: Executable + 'static>(&mut self, node: T) {
        let id = node.get_base().id.clone();
        self.nodes.insert(id, Arc::new(node));
    }

    /// 添加边
    pub fn add_edge(&mut self, start: &str, end: &str) -> Result<()> {
        if !self.nodes.contains_key(start) {
            return Err(Error::NodeNotFound(start.to_string()));
        }
        if !self.nodes.contains_key(end) {
            return Err(Error::NodeNotFound(end.to_string()));
        }

        self.edges.push(Edge {
            start: start.to_string(),
            end: end.to_string(),
        });

        Ok(())
    }

    /// 编译图：检查循环依赖并构建前置/后继节点关系
    pub fn compile(&mut self) -> Result<Context> {
        self.predecessors.clear();
        self.successors.clear();

        let mut in_degree: HashMap<String, usize> = HashMap::new();

        for node_id in self.nodes.keys() {
            in_degree.insert(node_id.clone(), 0);
        }

        for edge in &self.edges {
            if !self.nodes.contains_key(&edge.start) || !self.nodes.contains_key(&edge.end) {
                return Err(Error::ExecutionError(format!(
                    "Invalid edge from {} to {}",
                    edge.start, edge.end
                )));
            }

            self.successors
                .entry(edge.start.clone())
                .or_default()
                .insert(edge.end.clone());
            self.predecessors
                .entry(edge.end.clone())
                .or_default()
                .insert(edge.start.clone());

            *in_degree.entry(edge.end.clone()).or_insert(0) += 1;
        }

        // 拓扑排序检查循环依赖
        let mut queue = VecDeque::new();
        for (node, &deg) in &in_degree {
            if deg == 0 {
                queue.push_back(node.clone());
            }
        }

        let mut visited_count = 0;

        while let Some(current) = queue.pop_front() {
            visited_count += 1;

            if let Some(children) = self.successors.get(&current) {
                for child in children {
                    let deg = in_degree.get_mut(child).unwrap();
                    *deg -= 1;

                    if *deg == 0 {
                        queue.push_back(child.clone());
                    }
                }
            }
        }

        if visited_count != self.nodes.len() {
            return Err(Error::ExecutionError(
                "Cycle detected in graph!".to_string(),
            ));
        }

        // 填充空节点
        for node_id in self.nodes.keys() {
            self.successors
                .entry(node_id.clone())
                .or_insert_with(HashSet::new);
            self.predecessors
                .entry(node_id.clone())
                .or_insert_with(HashSet::new);
        }

        self.compiled = true;
        Ok(Context::new(&self.nodes))
    }

    /// 获取节点
    pub fn get_node(&self, node_id: &str) -> Option<Arc<dyn Executable>> {
        self.nodes.get(node_id).cloned()
    }

    /// 序列化为 JSON 字符串（仅保存 edges，不保存 nodes）
    pub fn to_json(&self) -> String {
        let data = serde_json::json!({
            "edges": self.edges
        });

        serde_json::to_string_pretty(&data).expect("Failed to serialize Graph")
    }

    /// 从 JSON 字符串反序列化，仅恢复 edges，不恢复 nodes
    pub fn from_json(json: &str) -> Result<Self> {
        let data: serde_json::Value = serde_json::from_str(json)?;
        let edges = data["edges"]
            .as_array()
            .ok_or_else(|| Error::ExecutionError("Invalid edges data".to_string()))?;

        let edges: Vec<Edge> = serde_json::from_value(serde_json::Value::Array(edges.clone()))?;
        let mut graph = Graph::new();
        graph.edges = edges;

        Ok(graph)
    }
}
