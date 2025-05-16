use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::{
    edge::{Edge, EdgeType},
    error::{Error, Result},
    model::{graph_data::GraphData, node::Node},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Graph {
    /// 节点数据：持久化存储，保存节点的静态配置信息
    pub nodes: HashMap<String, Node>,

    /// 边信息：节点之间的依赖关系
    pub edges: Vec<Edge>,

    /// 起始节点和结束节点（虚拟节点，不计入 node_data）
    pub start_node: Option<String>,
    pub end_node: Option<String>,

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
            nodes: HashMap::new(),
            edges: Vec::new(),
            start_node: None,
            end_node: None,
            compiled: false,
            predecessors: HashMap::new(),
            successors: HashMap::new(),
        }
    }

    fn mark_uncompiled(&mut self) {
        self.compiled = false;
    }

    pub fn set_start_node(&mut self, node: Node) -> Result<()> {
        if self.start_node.is_some() {
            return Err(Error::NodeAlreadyExists(node.id));
        }
        let node_id = node.id.clone();
        self.add_node(node)?;
        self.start_node = Some(node_id);
        Ok(())
    }

    pub fn set_end_node(&mut self, node: Node) -> Result<()> {
        if self.end_node.is_some() {
            return Err(Error::NodeAlreadyExists(node.id));
        }
        let node_id = node.id.clone();
        self.add_node(node)?;
        self.end_node = Some(node_id);
        Ok(())
    }

    /// 添加节点到持久化数据
    pub fn add_node(&mut self, node: Node) -> Result<()> {
        if self.nodes.contains_key(&node.id) {
            return Err(Error::NodeAlreadyExists(node.id.clone()));
        }
        self.nodes.insert(node.id.clone(), node);
        self.mark_uncompiled();
        Ok(())
    }

    /// 更新节点数据
    pub fn update_node(&mut self, node: Node) -> Result<()> {
        if !self.nodes.contains_key(&node.id) {
            return Err(Error::NodeNotFound(node.id.clone()));
        }

        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// 删除节点
    pub fn remove_node(&mut self, node_id: &str) -> Result<()> {
        if !self.nodes.contains_key(node_id) {
            return Err(Error::NodeNotFound(node_id.to_string()));
        }

        // 移除节点数据
        self.nodes.remove(node_id);

        // 移除相关边（起点或终点为该节点的边）
        self.edges
            .retain(|edge| edge.start != node_id && edge.end != node_id);

        // 检查并重置 start_node 和 end_node
        if self.start_node.as_deref() == Some(node_id) {
            self.start_node = None;
        }

        if self.end_node.as_deref() == Some(node_id) {
            self.end_node = None;
        }

        // 标记未编译状态，compile 时会重新生成 predecessors 和 successors
        self.mark_uncompiled();
        Ok(())
    }

    /// 添加边，自动推断 edge_type
    pub fn add_edge(&mut self, start: &str, end: &str) -> Result<()> {
        if self.end_node.as_deref() == Some(start) {
            return Err(Error::ExecutionError(
                "End node cannot have outgoing edges.".to_string(),
            ));
        }

        if self.start_node.as_deref() == Some(end) {
            return Err(Error::ExecutionError(
                "Start node cannot have incoming edges.".to_string(),
            ));
        }

        if !self.nodes.contains_key(start) {
            return Err(Error::NodeNotFound(start.to_string()));
        }
        if !self.nodes.contains_key(end) {
            return Err(Error::NodeNotFound(end.to_string()));
        }

        let start_node = self.nodes.get(start).unwrap();
        let end_node = self.nodes.get(end).unwrap();

        let edge_type = if start_node.is_control_node() {
            // 控制节点出口，只能连接到数据节点
            if end_node.is_control_node() {
                return Err(Error::ExecutionError(format!(
                    "Control node '{}' cannot connect to another control node '{}'",
                    start, end
                )));
            }
            EdgeType::Control
        } else {
            // 数据节点出口，可以连接到控制节点或数据节点
            EdgeType::Data
        };

        self.edges.push(Edge {
            start: start.to_string(),
            end: end.to_string(),
            edge_type,
        });

        Ok(())
    }

    fn topological_sort(&self) -> Result<Vec<String>> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut sorted_nodes = Vec::new();

        // 初始化入度计数器
        for node_id in self.nodes.keys() {
            in_degree.insert(node_id.clone(), 0);
        }

        // 计算入度
        for edge in &self.edges {
            *in_degree.entry(edge.end.clone()).or_insert(0) += 1;
        }

        // 将入度为 0 的节点入队
        for (node, &deg) in &in_degree {
            if deg == 0 {
                queue.push_back(node.clone());
            }
        }

        // 拓扑排序
        while let Some(current) = queue.pop_front() {
            sorted_nodes.push(current.clone());

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

        // 检查循环依赖
        if sorted_nodes.len() != self.nodes.len() {
            return Err(Error::ExecutionError(
                "Cycle detected in graph!".to_string(),
            ));
        }

        Ok(sorted_nodes)
    }

    /// 编译图：检查循环依赖并构建前置/后继节点关系
    pub fn compile(&mut self) -> Result<()> {
        self.predecessors.clear();
        self.successors.clear();

        for edge in &self.edges {
            if !self.nodes.contains_key(&edge.start) || !self.nodes.contains_key(&edge.end) {
                return Err(Error::ExecutionError(format!(
                    "Invalid edge from {} to {}",
                    edge.start, edge.end
                )));
            }

            // 构建前置/后继节点关系
            self.successors
                .entry(edge.start.clone())
                .or_default()
                .insert(edge.end.clone());

            self.predecessors
                .entry(edge.end.clone())
                .or_default()
                .insert(edge.start.clone());
        }

        // 调用拓扑排序方法
        self.topological_sort()?;
        self.compiled = true;
        Ok(())
    }

    /// 序列化为 JSON 字符串
    pub fn to_json(&self) -> String {
        let graph_data = GraphData {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        };
        serde_json::to_string_pretty(&graph_data).expect("Failed to serialize Graph")
    }

    /// 从 JSON 字符串反序列化
    pub fn from_json(json: &str) -> Result<Self> {
        let graph_data: GraphData = serde_json::from_str(json)?;
        let mut graph = Graph::new();
        graph.nodes = graph_data.nodes;
        graph.edges = graph_data.edges;
        Ok(graph)
    }
}
