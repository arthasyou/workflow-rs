use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    edge::Edge,
    node::{Node, NodeKind},
};

#[derive(Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
    pub compiled: bool,

    pub predecessors: HashMap<String, HashSet<String>>, // 节点 -> 它的前置节点(可以有多个)
    pub successors: HashMap<String, HashSet<String>>,   // 节点 -> 它的后继节点(可以有多个)
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            compiled: false,
            predecessors: HashMap::new(),
            successors: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_edge(&mut self, start: &str, end: &str) -> Result<(), String> {
        if !self.nodes.contains_key(start) {
            return Err(format!("Start node '{}' not found", start));
        }
        if !self.nodes.contains_key(end) {
            return Err(format!("End node '{}' not found", end));
        }

        self.edges.push(Edge {
            start: start.to_string(),
            end: end.to_string(),
        });

        Ok(())
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Failed to serialize Graph")
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn compile(&mut self) -> Result<(), String> {
        // 1. 清空旧的 predecessors, successors
        self.predecessors.clear();
        self.successors.clear();

        // 2. 初始化节点入度表
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for key in self.nodes.keys() {
            in_degree.insert(key.clone(), 0);
        }

        // 3. 建立 successors、predecessors、in_degree(入度表)
        for edge in &self.edges {
            // 检查节点存在
            if !self.nodes.contains_key(&edge.start) || !self.nodes.contains_key(&edge.end) {
                return Err(format!("Invalid edge from {} to {}", edge.start, edge.end));
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

        // 4. Kahn 拓扑排序：检查是否有环
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
            return Err("Cycle detected in graph!".into());
        }

        // 5. 确保每个节点都在 successors 和 predecessors 中注册
        for id in self.nodes.keys() {
            self.successors
                .entry(id.clone())
                .or_insert_with(HashSet::new);
            self.predecessors
                .entry(id.clone())
                .or_insert_with(HashSet::new);
        }

        self.compiled = true;
        Ok(())
    }

    pub fn invoke(&self, input: Value) -> Result<Value, String> {
        if !self.compiled {
            return Err("Graph must be compiled before invoke.".into());
        }

        // 初始化每个节点当前待处理的前置节点数量
        let mut pending_predecessors: HashMap<String, usize> = HashMap::new();
        for id in self.nodes.keys() {
            let preds = self.predecessors.get(id).map_or(0, |s| s.len());
            pending_predecessors.insert(id.clone(), preds);
        }

        // 输入数据，按节点ID保存节点执行结果
        let mut node_outputs: HashMap<String, Value> = HashMap::new();

        // 先把输入数据给 start nodes
        let mut queue = VecDeque::new();
        for (id, &count) in &pending_predecessors {
            if count == 0 {
                queue.push_back(id.clone());
                node_outputs.insert(id.clone(), input.clone()); // 初始数据给start nodes
            }
        }

        while let Some(current) = queue.pop_front() {
            let current_output = node_outputs.get(&current).cloned().unwrap_or(Value::Null);

            // 执行当前节点逻辑（这里先简单模拟：把当前输出+节点id打包）
            let result = self.execute_node(&current, current_output)?;

            node_outputs.insert(current.clone(), result.clone());

            // 处理后继节点
            if let Some(next_nodes) = self.successors.get(&current) {
                for next in next_nodes {
                    // 前置节点执行完了，减少依赖计数
                    if let Some(pending) = pending_predecessors.get_mut(next) {
                        *pending -= 1;
                        if *pending == 0 {
                            queue.push_back(next.clone());
                        }
                    }
                }
            }
        }

        // 找 end nodes
        let mut end_nodes: Vec<String> = vec![];
        for (id, succ) in &self.successors {
            if succ.is_empty() {
                end_nodes.push(id.clone());
            }
        }

        if end_nodes.is_empty() {
            return Err("No end node found.".into());
        }

        // 返回其中一个 end node 的输出
        let output = node_outputs
            .get(&end_nodes[0])
            .cloned()
            .unwrap_or(Value::Null);
        Ok(output)
    }
}

impl Graph {
    fn execute_node(&self, node_id: &str, input: Value) -> Result<Value, String> {
        let node = self
            .nodes
            .get(node_id)
            .ok_or(format!("Node {} not found", node_id))?;

        match node.kind {
            NodeKind::Prompt => {
                if let Some(config) = &node.config {
                    if let Some(template) = config.get("template").and_then(|v| v.as_str()) {
                        let filled = template.replace("{input}", input.as_str().unwrap_or(""));
                        Ok(Value::String(filled))
                    } else {
                        Ok(Value::String(format!("Prompted: {}", input)))
                    }
                } else {
                    Ok(Value::String(format!("Prompted: {}", input)))
                }
            }
            NodeKind::Model => {
                if let Some(config) = &node.config {
                    if let Some(model_name) = config.get("model_name").and_then(|v| v.as_str()) {
                        Ok(Value::String(format!(
                            "Model({}) output based on: {}",
                            model_name, input
                        )))
                    } else {
                        Ok(Value::String(format!("Model output based on: {}", input)))
                    }
                } else {
                    Ok(Value::String(format!("Model output based on: {}", input)))
                }
            }
            NodeKind::Retriever => Ok(Value::String(format!("Retrieved info for: {}", input))),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        graph::Graph,
        node::{Node, NodeKind},
        storage::{GraphStorage, mock::MockStorage},
    };

    #[test]
    fn save_and_load_graph_mock_storage() {
        let mut g = Graph::new();
        g.add_node(Node {
            id: "prompt".into(),
            kind: NodeKind::Prompt,
            config: None,
        });
        g.add_node(Node {
            id: "model".into(),
            kind: NodeKind::Model,
            config: None,
        });
        let _ = g.add_edge("prompt", "model");

        let storage = MockStorage::new();

        storage
            .save_graph("test_graph", &g)
            .expect("Failed to save");

        let loaded_graph = storage.load_graph("test_graph").expect("Failed to load");

        assert_eq!(loaded_graph.nodes.len(), 2);
        assert_eq!(loaded_graph.edges.len(), 1);

        println!("Loaded Graph Nodes: {:?}", loaded_graph.nodes.keys());
    }

    #[test]
    fn compile_builds_successors_and_predecessors() {
        let mut g = Graph::new();
        g.add_node(Node {
            id: "a".into(),
            kind: NodeKind::Prompt,
            config: None,
        });
        g.add_node(Node {
            id: "b".into(),
            kind: NodeKind::Model,
            config: None,
        });
        let _ = g.add_edge("a", "b");

        g.compile().expect("Should compile");

        assert_eq!(g.successors.get("a").unwrap().contains("b"), true);
        assert_eq!(g.predecessors.get("b").unwrap().contains("a"), true);
    }

    #[test]
    fn test_invoke_simple_graph() {
        let mut g = Graph::new();
        g.add_node(Node {
            id: "prompt".into(),
            kind: NodeKind::Prompt,
            config: None,
        });
        g.add_node(Node {
            id: "model".into(),
            kind: NodeKind::Model,
            config: None,
        });

        let _ = g.add_edge("prompt", "model");

        g.compile().expect("Compile failed");

        let input = Value::String("What's the weather?".into());
        let output = g.invoke(input).expect("Invoke failed");

        println!("Final output: {}", output);
    }

    #[test]
    fn test_execute_node_with_config() {
        let mut g = Graph::new();

        g.add_node(Node {
            id: "prompt".into(),
            kind: NodeKind::Prompt,
            config: Some(serde_json::json!({
                "template": "Hello, {input}! Welcome!"
            })),
        });

        let input = Value::String("Alice".into());
        g.compile().unwrap();

        let output = g.invoke(input).unwrap();
        println!("Test output: {}", output);
    }
}
