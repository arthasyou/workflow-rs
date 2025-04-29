use std::collections::{HashMap, HashSet, VecDeque};

use serde_json::Value;

use crate::{graph::Graph, types::Executable};

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

    pub fn run(&mut self, input: Value) -> Result<Value, String> {
        if !self.graph.compiled {
            return Err("Graph must be compiled before running.".into());
        }

        // 执行逻辑（和 graph.invoke 逻辑一致）
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
            let node = self.graph.nodes.get(&current).ok_or("Node not found")?;
            let output = node.execute(input.clone())?;
            self.outputs.insert(current.clone(), output.clone());

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

        // 找到 end 节点
        let end_nodes: Vec<_> = self
            .graph
            .successors
            .iter()
            .filter(|(_, succs)| succs.is_empty())
            .map(|(id, _)| id.clone())
            .collect();

        if end_nodes.is_empty() {
            return Err("No end node found.".into());
        }

        Ok(self
            .outputs
            .get(&end_nodes[0])
            .cloned()
            .unwrap_or(Value::Null))
    }
}
