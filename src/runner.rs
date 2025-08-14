use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use flow_data::{
    FlowData, FlowOutputType,
    output::{ControlFlow, FlowOutput},
};
use workflow_error::{Error, Result};
use workflow_utils::stream_util::forward_and_collect_stream;

use crate::{graph::Graph, model::Context, types::StreamSender};

/// Runner 负责调度节点执行，管理节点间的数据传递与控制流
pub struct Runner {
    inputs: HashMap<String, FlowData>,
    outputs: HashMap<String, FlowData>,
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
    pub fn set_input(&mut self, node_id: &str, input: Option<FlowData>) {
        match input {
            Some(data) => {
                self.inputs.insert(node_id.to_string(), data);
            }
            None => return,
        }
    }

    /// 获取输入数据
    pub fn get_input(&self, node_id: &str) -> Result<&FlowData> {
        self.inputs
            .get(node_id)
            .ok_or_else(|| Error::NodeNotFound(node_id.to_string().into()))
    }

    /// 设置输出数据
    pub fn set_output(&mut self, node_id: &str, payload: FlowData) {
        self.outputs.insert(node_id.to_string(), payload);
    }

    /// 获取输出数据
    pub fn get_output(&self, node_id: &str) -> Result<&FlowData> {
        self.outputs
            .get(node_id)
            .ok_or_else(|| Error::NodeNotFound(node_id.to_string().into()))
    }

    pub fn get_resolved_input(&self, node_id: &str) -> Option<FlowData> {
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
    pub async fn run(
        &mut self,
        input: Option<FlowData>,
        graph: &mut Graph,
        stream_tx: Option<StreamSender>,
    ) -> Result<FlowData> {
        graph.compile()?;
        let context = Context::from_graph(graph);
        self.prepare(graph, input)?;
        self.execute_all_nodes(graph, context, stream_tx).await?;
        let output = self.get_output("end")?;
        Ok(output.clone())
    }

    /// 初始化节点状态
    fn prepare(&mut self, graph: &Graph, mut input: Option<FlowData>) -> Result<()> {
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
                // 只把 input 塞给第一个没有前驱的节点
                if let Some(data) = input.take() {
                    self.inputs.insert(node_id.clone(), data);
                }

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
    async fn execute_all_nodes(
        &mut self,
        graph: &Graph,
        context: Arc<Context>,
        stream_tx: Option<StreamSender>,
    ) -> Result<()> {
        while let Some(current) = self.queue.pop_front() {
            let input_value = self.get_resolved_input(&current);

            let node = context
                .get_node(&current)
                .ok_or_else(|| Error::NodeNotFound(current.clone().into()))?;

            let output = node.execute(input_value, context.clone()).await?;

            self.handle_output(&current, output, graph, &context, stream_tx.clone())
                .await?;
        }

        Ok(())
    }

    /// 处理节点输出
    async fn handle_output(
        &mut self,
        current: &str,
        output: FlowOutput,
        graph: &Graph,
        context: &Arc<Context>,
        stream_tx: Option<StreamSender>,
    ) -> Result<()> {
        match output.get_type() {
            FlowOutputType::Control => {
                let controll = output.into_control()?;
                self.handle_control_output(current, controll, graph, context)?;
            }
            FlowOutputType::Data => {
                let data = output.into_data()?;
                self.handle_data_output(current, data, graph)?;
            }
            FlowOutputType::Parallel => {
                self.handle_parallel_output();
            }
            FlowOutputType::Stream => {
                self.handle_stream_output(stream_tx, current, output)
                    .await?;
            }
        }
        Ok(())
    }

    fn handle_control_output(
        &mut self,
        current: &str,
        controll: ControlFlow,
        graph: &Graph,
        context: &Arc<Context>,
    ) -> Result<()> {
        let next_node_id = graph
            .handle_routes
            .get(&(current.to_owned(), controll.next_node.clone()))
            .ok_or_else(|| {
                Error::ExecutionError(
                    format!(
                        "No target node found for source '{}' with handle '{}'",
                        current, controll.next_node
                    )
                    .into(),
                )
            })?;

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
        self.set_output(current, controll.data.clone());
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
        data_payload: FlowData,
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

    async fn handle_stream_output(
        &self,
        stream_tx: Option<StreamSender>,
        current: &str,
        output: FlowOutput,
    ) -> Result<()> {
        if let Some(tx) = stream_tx.clone() {
            let stream = output.into_stream()?;
            let _r = forward_and_collect_stream(stream, tx, current).await?;
        }

        Ok(())
    }
}

// /// 合并两个 `DataPayload` 数据，用于累积多个输入数据。
// pub fn merge_inputs(existing: DataPayload, new_data: DataPayload) -> DataPayload {
//     let combined = existing.merge(new_data);
//     combined
// }

// /// 检查节点是否是控制节点 (如 Branch, Repeat, Parallel)
// pub fn is_control_node(node: &Node) -> bool {
//     node.is_control_node()
// }
