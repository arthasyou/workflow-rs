use std::collections::HashMap;

use crate::{edge::Edge, node::Node};

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
}
