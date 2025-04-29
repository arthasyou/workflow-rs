use std::collections::{HashMap, VecDeque};

use serde_json::Value;

use crate::{
    error::{Error, Result},
    graph::Graph,
    node::NodeKind,
    types::Executable,
};

pub struct Runner<'a> {
    graph: &'a Graph,
    outputs: HashMap<String, Value>, // 每个节点的执行结果
}

impl<'a> Runner<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            outputs: HashMap::new(),
        }
    }

    pub fn run(&mut self, input: Value) -> Result<Value> {
        if !self.graph.compiled {
            return Err(Error::GraphNotCompiled);
        }

        let mut pending_predecessors: HashMap<String, usize> = HashMap::new();
        for id in self.graph.nodes.keys() {
            let preds = self.graph.predecessors.get(id).map_or(0, |s| s.len());
            pending_predecessors.insert(id.clone(), preds);
        }

        let mut queue = VecDeque::new();
        for (id, &count) in &pending_predecessors {
            if count == 0 {
                queue.push_back(id.clone());
                self.outputs.insert(id.clone(), input.clone());
            }
        }

        while let Some(current) = queue.pop_front() {
            let input = self.outputs.get(&current).cloned().unwrap_or(Value::Null);

            let node = self
                .graph
                .nodes
                .get(&current)
                .ok_or_else(|| Error::NodeNotFound(current.clone()))?;

            let output = node.execute(input.clone())?;
            self.outputs.insert(current.clone(), output.clone());

            match node.kind {
                NodeKind::Branch => {
                    // 🎯 Branch节点，动态跳转
                    let next_node_id = output.as_str().ok_or(Error::InvalidBranchInput)?;
                    if !self.graph.nodes.contains_key(next_node_id) {
                        return Err(Error::NodeNotFound(next_node_id.to_string()));
                    }
                    queue.push_back(next_node_id.to_string());
                }
                _ => {
                    // 🎯 普通节点，按照 successors 正常走
                    if let Some(next_nodes) = self.graph.successors.get(&current) {
                        for next in next_nodes {
                            if let Some(pending) = pending_predecessors.get_mut(next) {
                                *pending -= 1;
                                if *pending == 0 {
                                    queue.push_back(next.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        let end_nodes: Vec<_> = self
            .graph
            .successors
            .iter()
            .filter(|(_, succs)| succs.is_empty())
            .map(|(id, _)| id.clone())
            .collect();

        if end_nodes.is_empty() {
            return Err(Error::NoEndNode);
        }

        Ok(self
            .outputs
            .get(&end_nodes[0])
            .cloned()
            .unwrap_or(Value::Null))
    }
}
