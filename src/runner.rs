use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use workflow_error::{Error, Result};

use crate::{
    graph::Graph,
    model::{Context, DataPayload, Node, OutputData},
};

/// Runner 负责调度节点执行，管理节点间的数据传递与控制流
pub struct Runner {
    inputs: HashMap<String, DataPayload>,  // 每个节点的输入数据
    outputs: HashMap<String, DataPayload>, // 每个节点的输出数据
    input_refs: HashMap<String, String>,
    queue: VecDeque<String>,
    pending_predecessors: HashMap<String, usize>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            input_refs: HashMap::new(),
            queue: VecDeque::new(),
            pending_predecessors: HashMap::new(),
        }
    }

    /// 设置输入数据
    pub fn set_input(&mut self, node_id: &str, input: DataPayload) {
        self.inputs.insert(node_id.to_string(), input);
    }

    /// 获取输入数据
    pub fn get_input(&self, node_id: &str) -> Result<&DataPayload> {
        self.inputs
            .get(node_id)
            .ok_or_else(|| Error::NodeNotFound(node_id.to_string()))
    }

    /// 设置输出数据
    pub fn set_output(&mut self, node_id: &str, payload: DataPayload) {
        self.outputs.insert(node_id.to_string(), payload);
    }

    /// 获取输出数据
    pub fn get_output(&self, node_id: &str) -> Result<&DataPayload> {
        self.outputs
            .get(node_id)
            .ok_or_else(|| Error::NodeNotFound(node_id.to_string()))
    }

    pub fn get_resolved_input(&self, node_id: &str) -> Option<DataPayload> {
        if let Some(data) = self.inputs.get(node_id) {
            return Some(data.clone());
        }
        if let Some(source_id) = self.input_refs.get(node_id) {
            if let Some(data) = self.outputs.get(source_id) {
                return Some(data.clone());
            }
        }
        None
    }

    /// 运行图
    pub async fn run(&mut self, graph: &mut Graph) -> Result<DataPayload> {
        graph.compile()?;
        let context = Context::from_graph(graph);
        self.prepare(graph)?;
        self.execute_all_nodes(graph, context).await?;
        let output = self.get_output("end")?;
        Ok(output.clone())
    }

    /// 初始化节点状态
    fn prepare(&mut self, graph: &Graph) -> Result<()> {
        self.queue.clear();
        self.pending_predecessors.clear();
        self.inputs.clear();
        self.outputs.clear();
        self.input_refs.clear();

        for node_id in graph.nodes.keys() {
            let pred_count = graph.predecessors.get(node_id).map_or(0, |s| s.len());
            self.pending_predecessors
                .insert(node_id.clone(), pred_count);

            if pred_count == 0 {
                // self.inputs.insert(node_id.clone(), DataPayload::default());
                self.queue.push_back(node_id.clone());
            }
        }

        Ok(())
    }

    fn mark_branch_skipped(&mut self, node_id: &str, graph: &Graph) {
        if let Some(successors) = graph.successors.get(node_id) {
            for succ in successors {
                if let Some(pred_count) = self.pending_predecessors.get_mut(succ) {
                    if *pred_count > 0 {
                        *pred_count -= 1;
                    }
                }
            }
        }
    }

    /// 执行所有节点
    async fn execute_all_nodes(&mut self, graph: &Graph, context: Arc<Context>) -> Result<()> {
        while let Some(current) = self.queue.pop_front() {
            let input_value = self.get_resolved_input(&current);

            let node = context
                .get_node(&current)
                .ok_or_else(|| Error::NodeNotFound(current.clone()))?;

            let output = node.execute(input_value, context.clone()).await?;

            self.handle_output(&current, output, graph, &context)?;
        }

        Ok(())
    }

    /// 处理节点输出
    fn handle_output(
        &mut self,
        current: &str,
        output: OutputData,
        graph: &Graph,
        context: &Arc<Context>,
    ) -> Result<()> {
        match output {
            OutputData::Control(next_node_id) => {
                self.handle_control_output(current, &next_node_id, graph, context)?;
            }
            OutputData::Data(data_payload) => {
                self.handle_data_output(current, data_payload, graph)?;
            }
            OutputData::Parallel(_node_outputs) => {
                self.handle_parallel_output();
            }
        }
        Ok(())
    }

    fn handle_control_output(
        &mut self,
        current: &str,
        next_node_id: &str,
        graph: &Graph,
        context: &Arc<Context>,
    ) -> Result<()> {
        if let Some(successors) = graph.successors.get(current) {
            for succ in successors {
                if let Some(p) = self.pending_predecessors.get_mut(succ) {
                    *p -= 1;
                }
                if succ != next_node_id {
                    self.mark_branch_skipped(succ, graph);
                }
            }
        }
        if context.get_node(next_node_id).is_some() {
            self.queue.push_back(next_node_id.to_string());
            self.input_refs
                .insert(next_node_id.to_string(), current.to_string());
        }
        Ok(())
    }

    fn handle_data_output(
        &mut self,
        current: &str,
        data_payload: DataPayload,
        graph: &Graph,
    ) -> Result<()> {
        self.set_output(current, data_payload.clone());
        if let Some(successors) = graph.successors.get(current) {
            for next_node_id in successors {
                let pred_count = self
                    .pending_predecessors
                    .get_mut(next_node_id)
                    .expect("Expected predecessor count");
                if *pred_count > 0 {
                    *pred_count -= 1;
                }
                self.input_refs
                    .insert(next_node_id.clone(), current.to_string());
                if *pred_count == 0 {
                    self.queue.push_back(next_node_id.clone());
                }
            }
        }
        Ok(())
    }

    fn handle_parallel_output(&self) {
        todo!()
    }
}

/// 合并两个 `DataPayload` 数据，用于累积多个输入数据。
pub fn merge_inputs(existing: DataPayload, new_data: DataPayload) -> DataPayload {
    let combined = existing.merge(new_data);
    combined
}

/// 检查节点是否是控制节点 (如 Branch, Repeat, Parallel)
pub fn is_control_node(node: &Node) -> bool {
    node.is_control_node()
}
