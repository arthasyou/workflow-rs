use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{edge::Edge, node::Node};

#[derive(Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
    pub compiled: bool,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            compiled: false,
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_edge(&mut self, start: &str, end: &str) {
        // 这里只简单检查是否存在节点
        if !self.nodes.contains_key(start) {
            panic!("Start node {} not found", start);
        }
        if !self.nodes.contains_key(end) {
            panic!("End node {} not found", end);
        }

        self.edges.push(Edge {
            start: start.to_string(),
            end: end.to_string(),
        });
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Failed to serialize Graph")
    }
}

impl Graph {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
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
        });
        g.add_node(Node {
            id: "model".into(),
            kind: NodeKind::Model,
        });
        g.add_edge("prompt", "model");

        let storage = MockStorage::new();

        storage
            .save_graph("test_graph", &g)
            .expect("Failed to save");

        let loaded_graph = storage.load_graph("test_graph").expect("Failed to load");

        assert_eq!(loaded_graph.nodes.len(), 2);
        assert_eq!(loaded_graph.edges.len(), 1);

        println!("Loaded Graph Nodes: {:?}", loaded_graph.nodes.keys());
    }
}
