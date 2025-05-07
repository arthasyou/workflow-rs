use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    edge::Edge,
    error::{Error, Result},
    node::Node,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
    pub compiled: bool,

    pub predecessors: HashMap<String, HashSet<String>>, // 节点 -> 它的前置节点
    pub successors: HashMap<String, HashSet<String>>,   // 节点 -> 它的后继节点
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

    pub fn compile(&mut self) -> Result<()> {
        self.predecessors.clear();
        self.successors.clear();

        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for key in self.nodes.keys() {
            in_degree.insert(key.clone(), 0);
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

        use std::collections::VecDeque;
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

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Failed to serialize Graph")
    }

    pub fn from_json(json: &str) -> std::result::Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
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
            input_processor: None,
            output_processor: None,
        });
        g.add_node(Node {
            id: "model".into(),
            kind: NodeKind::Model,
            config: None,
            input_processor: None,
            output_processor: None,
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
            input_processor: None,
            output_processor: None,
        });
        g.add_node(Node {
            id: "b".into(),
            kind: NodeKind::Model,
            config: None,
            input_processor: None,
            output_processor: None,
        });
        let _ = g.add_edge("a", "b");

        g.compile().expect("Should compile");

        assert_eq!(g.successors.get("a").unwrap().contains("b"), true);
        assert_eq!(g.predecessors.get("b").unwrap().contains("a"), true);
    }
}
