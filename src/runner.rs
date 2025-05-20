use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use crate::{
    error::{Error, Result},
    graph::Graph,
    model::{Context, DataPayload, OutputData},
};

/// Runner 负责调度节点执行，管理节点间的数据传递与控制流
pub struct Runner {
    inputs: HashMap<String, DataPayload>, // 每个节点的输入数据
    outputs: HashMap<String, OutputData>, // 每个节点的输出数据
    queue: VecDeque<String>,
    pending_predecessors: HashMap<String, usize>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
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
    pub fn set_output(&mut self, node_id: &str, output: OutputData) {
        self.outputs.insert(node_id.to_string(), output);
    }

    /// 获取输出数据
    pub fn get_output(&self, node_id: &str) -> Result<&OutputData> {
        self.outputs
            .get(node_id)
            .ok_or_else(|| Error::NodeNotFound(node_id.to_string()))
    }

    /// 运行图
    pub async fn run(&mut self, graph: &mut Graph, input: DataPayload) -> Result<()> {
        graph.compile()?;
        let context = Context::from_graph(graph);
        self.prepare(graph, input)?;
        self.execute_all_nodes(graph, context).await?;
        Ok(())
    }

    /// 初始化节点状态
    fn prepare(&mut self, graph: &Graph, input: DataPayload) -> Result<()> {
        self.queue.clear();
        self.pending_predecessors.clear();
        self.inputs.clear();
        self.outputs.clear();

        for node_id in graph.nodes.keys() {
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
    async fn execute_all_nodes(&mut self, graph: &Graph, context: Arc<Context>) -> Result<()> {
        while let Some(current) = self.queue.pop_front() {
            let input_value = self.get_input(&current)?;

            let node = context
                .get_node(&current)
                .ok_or_else(|| Error::NodeNotFound(current.clone()))?;

            let output = node.execute(input_value.clone(), context.clone()).await?;

            match output {
                OutputData::Control(next_node_id) => {
                    if context.get_node(&next_node_id).is_some() {
                        self.queue.push_back(next_node_id.clone());
                    }
                }
                OutputData::Data(data_payload) => {
                    self.set_output(&current, OutputData::Data(data_payload.clone()));

                    // 直接从 `graph.successors` 读取后继节点
                    if let Some(successors) = graph.successors.get(&current) {
                        for next_node_id in successors {
                            let pred_count =
                                self.pending_predecessors.get_mut(next_node_id).unwrap();
                            if *pred_count > 0 {
                                *pred_count -= 1;
                            }

                            if *pred_count == 0 {
                                self.queue.push_back(next_node_id.clone());
                            }
                        }
                    }
                }
                OutputData::Parallel(node_outputs) => todo!(),
            }
        }

        Ok(())
    }
}
