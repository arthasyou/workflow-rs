use std::collections::{HashMap, HashSet, VecDeque};

use serde_json::Value;

use crate::{
    error::{Error, Result},
    graph::Graph,
    // node::Executable,
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

    /// 初始化节点状态，计算前驱节点数量
    fn prepare(&mut self, input: Value) -> Result<()> {
        if !self.graph.compiled {
            return Err(Error::GraphNotCompiled);
        }

        self.outputs.clear();
        self.queue.clear();
        self.executed.clear();
        self.pending_predecessors.clear();

        for id in self.graph.nodes.keys() {
            let pred_count = self.graph.predecessors.get(id).map_or(0, |s| s.len());
            self.pending_predecessors.insert(id.clone(), pred_count);
        }

        // 初始化队列
        for (id, &count) in &self.pending_predecessors {
            if count == 0 {
                self.queue.push_back(id.clone());
                self.outputs.insert(id.clone(), input.clone());
            }
        }

        Ok(())
    }

    /// 执行所有节点
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

            // 执行节点并获取输出
            let output = node.execute(input_value.clone())?;
            self.outputs.insert(current.clone(), output.clone());

            // 处理节点后继节点
            self.process_successors(&current, output)?;
        }

        Ok(())
    }

    /// 处理节点的后继节点
    fn process_successors(&mut self, current: &str, output: Value) -> Result<()> {
        if let Some(successors) = self.graph.successors.get(current) {
            for next_id in successors {
                // 检查是否已经执行过
                if self.executed.contains(next_id) {
                    continue;
                }

                // 更新前驱节点数量
                if let Some(pending) = self.pending_predecessors.get_mut(next_id) {
                    *pending -= 1;

                    // 前驱节点全部执行完毕，放入队列
                    if *pending == 0 {
                        self.queue.push_back(next_id.clone());
                    }
                }

                // 将当前节点的输出传递给后继节点
                self.outputs.insert(next_id.clone(), output.clone());
            }
        }

        Ok(())
    }

    /// 获取最终输出节点的结果
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

        let output = self
            .outputs
            .get(&end_nodes[0])
            .cloned()
            .unwrap_or(Value::Null);
        Ok(output)
    }
}
