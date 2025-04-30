use std::collections::{HashMap, HashSet, VecDeque};

use serde_json::Value;

use crate::{
    error::{Error, Result},
    graph::Graph,
    node::NodeKind,
    types::Executable,
};

pub struct Runner<'a> {
    graph: &'a Graph,
    outputs: HashMap<String, Value>,
    queue: VecDeque<String>,
    pending_predecessors: HashMap<String, usize>,
    executed: HashSet<String>,
}

impl<'a> Runner<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            outputs: HashMap::new(),
            queue: VecDeque::new(),
            pending_predecessors: HashMap::new(),
            executed: HashSet::new(),
        }
    }

    pub fn run(&mut self, input: Value) -> Result<Value> {
        self.prepare(input)?;
        self.execute_all_nodes()?;
        self.resolve_output()
    }

    fn prepare(&mut self, input: Value) -> Result<()> {
        if !self.graph.compiled {
            return Err(Error::GraphNotCompiled);
        }

        self.outputs.clear();
        self.queue.clear();
        self.executed.clear();
        self.pending_predecessors.clear();

        for id in self.graph.nodes.keys() {
            let preds = self.graph.predecessors.get(id).map_or(0, |s| s.len());
            self.pending_predecessors.insert(id.clone(), preds);
        }

        for (id, &count) in &self.pending_predecessors {
            if count == 0 {
                self.queue.push_back(id.clone());
                self.outputs.insert(id.clone(), input.clone());
            }
        }

        Ok(())
    }

    fn execute_all_nodes(&mut self) -> Result<()> {
        while let Some(current) = self.queue.pop_front() {
            if self.executed.contains(&current) {
                continue;
            }
            self.executed.insert(current.clone());

            let input_value = self.outputs.get(&current).cloned().unwrap_or(Value::Null);
            let node = self
                .graph
                .nodes
                .get(&current)
                .ok_or_else(|| Error::NodeNotFound(current.clone()))?;

            let output = node.execute(input_value.clone())?;
            self.outputs.insert(current.clone(), output.clone());

            match node.kind {
                NodeKind::Branch => {
                    let next_node_id = output.as_str().ok_or(Error::InvalidBranchInput)?;
                    if !self.graph.nodes.contains_key(next_node_id) {
                        return Err(Error::NodeNotFound(next_node_id.to_string()));
                    }
                    self.queue.push_back(next_node_id.to_string());
                    self.outputs
                        .insert(next_node_id.to_string(), input_value.clone());
                }
                _ => {
                    if let Some(next_nodes) = self.graph.successors.get(&current) {
                        for next in next_nodes {
                            if let Some(pending) = self.pending_predecessors.get_mut(next) {
                                *pending -= 1;
                                if *pending == 0 {
                                    self.queue.push_back(next.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn resolve_output(&self) -> Result<Value> {
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
