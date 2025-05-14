use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::{
    edge::Edge,
    error::{Error, Result},
    model::{graph_data::GraphData, node::Node},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Graph {
    /// 节点数据：持久化存储，保存节点的静态配置信息
    pub node_data: HashMap<String, Node>,

    /// 边信息：节点之间的依赖关系
    pub edges: Vec<Edge>,

    /// 编译状态
    pub compiled: bool,

    /// 前置节点与后继节点映射
    pub predecessors: HashMap<String, HashSet<String>>,
    pub successors: HashMap<String, HashSet<String>>,
}

impl Graph {
    /// 创建新的图结构
    pub fn new() -> Self {
        Self {
            node_data: HashMap::new(),
            edges: Vec::new(),
            compiled: false,
            predecessors: HashMap::new(),
            successors: HashMap::new(),
        }
    }

    /// 添加节点到持久化数据
    pub fn add_node_data(&mut self, node: Node) {
        self.node_data.insert(node.id.clone(), node);
    }

    /// 添加边
    pub fn add_edge(&mut self, start: &str, end: &str) -> Result<()> {
        if !self.node_data.contains_key(start) {
            return Err(Error::NodeNotFound(start.to_string()));
        }
        if !self.node_data.contains_key(end) {
            return Err(Error::NodeNotFound(end.to_string()));
        }

        self.edges.push(Edge {
            start: start.to_string(),
            end: end.to_string(),
        });

        Ok(())
    }

    /// 编译图：检查循环依赖并构建前置/后继节点关系
    pub fn compile(&mut self) -> Result<()> {
        self.predecessors.clear();
        self.successors.clear();

        let mut in_degree: HashMap<String, usize> = HashMap::new();

        for node_id in self.node_data.keys() {
            in_degree.insert(node_id.clone(), 0);
        }

        for edge in &self.edges {
            if !self.node_data.contains_key(&edge.start) || !self.node_data.contains_key(&edge.end)
            {
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

        if visited_count != self.node_data.len() {
            return Err(Error::ExecutionError(
                "Cycle detected in graph!".to_string(),
            ));
        }

        self.compiled = true;

        // 生成 Context（节点实例构建在 Context 内部处理）
        Ok(())
    }

    /// 序列化为 JSON 字符串
    pub fn to_json(&self) -> String {
        let graph_data = GraphData {
            nodes: self.node_data.clone(),
            edges: self.edges.clone(),
        };
        serde_json::to_string_pretty(&graph_data).expect("Failed to serialize Graph")
    }

    /// 从 JSON 字符串反序列化
    pub fn from_json(json: &str) -> Result<Self> {
        let graph_data: GraphData = serde_json::from_str(json)?;
        let mut graph = Graph::new();
        graph.node_data = graph_data.nodes;
        graph.edges = graph_data.edges;
        Ok(graph)
    }
}
