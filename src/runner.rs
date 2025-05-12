use std::collections::{HashMap, HashSet, VecDeque};

use serde_json::Value;

use crate::{
    error::{Error, Result},
    graph::Graph,
    model::Context,
};

pub struct Runner {
    inputs: HashMap<String, Vec<Value>>, // 每个节点的输入数据
    outputs: HashMap<String, Value>,     // 每个节点的输出数据
    queue: VecDeque<String>,
    pending_predecessors: HashMap<String, usize>,
    executed: HashSet<String>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            queue: VecDeque::new(),
            pending_predecessors: HashMap::new(),
            executed: HashSet::new(),
        }
    }

    /// 设置输入数据
    pub fn set_input(&mut self, node_id: &str, input: Value) {
        self.inputs
            .entry(node_id.to_string())
            .or_default()
            .push(input);
    }

    /// 获取输入数据
    pub fn get_input(&self, node_id: &str) -> Value {
        let inputs = self.inputs.get(node_id).cloned().unwrap_or_default();
        Value::Array(inputs)
    }

    /// 设置输出数据
    pub fn set_output(&mut self, node_id: &str, output: Value) {
        self.outputs.insert(node_id.to_string(), output);
    }

    /// 获取输出数据
    pub fn get_output(&self, node_id: &str) -> Option<&Value> {
        self.outputs.get(node_id)
    }

    /// 运行图
    pub async fn run(&mut self, graph: &Graph, context: &mut Context, input: Value) -> Result<()> {
        self.prepare(graph, input)?;
        self.execute_all_nodes(context).await
    }

    /// 初始化节点状态
    fn prepare(&mut self, graph: &Graph, input: Value) -> Result<()> {
        self.queue.clear();
        self.executed.clear();
        self.pending_predecessors.clear();
        self.inputs.clear();
        self.outputs.clear();

        for node_id in graph.node_data.keys() {
            let pred_count = graph.predecessors.get(node_id).map_or(0, |s| s.len());
            self.pending_predecessors
                .insert(node_id.clone(), pred_count);

            if pred_count == 0 {
                self.queue.push_back(node_id.clone());
                self.set_input(node_id, input.clone());
            }
        }

        Ok(())
    }

    /// 执行所有节点
    async fn execute_all_nodes(&mut self, context: &mut Context) -> Result<()> {
        while let Some(current) = self.queue.pop_front() {
            if self.executed.contains(&current) {
                continue;
            }

            self.executed.insert(current.clone());

            let input_value = self.get_input(&current);

            let node = context
                .get_node(&current)
                .ok_or_else(|| Error::NodeNotFound(current.clone()))?;

            let output = node.execute(input_value, context).await?;
            self.set_output(&current, output.clone());

            self.process_successors(&current, output)?;
        }

        Ok(())
    }

    /// 处理节点的后继节点
    fn process_successors(&mut self, _current: &str, output: Value) -> Result<()> {
        let next_ids: Vec<String> = self.pending_predecessors.keys().cloned().collect();

        for next_id in next_ids {
            if self.executed.contains(&next_id) {
                continue;
            }

            if let Some(pending) = self.pending_predecessors.get_mut(&next_id) {
                *pending -= 1;

                if *pending == 0 {
                    self.queue.push_back(next_id.clone());
                }
            }

            self.set_input(&next_id, output.clone());
        }

        Ok(())
    }
}
