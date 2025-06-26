use std::{collections::HashMap, sync::Arc};

use flow_data::{
    FlowData,
    output::{ControlFlow, FlowOutput},
};
use serde_json::Value;
use tokio::task::JoinSet;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase, config::ParallelConfig},
};

#[derive(Debug, Clone)]
pub struct ParallelNode {
    pub base: NodeBase,
    pub branches: HashMap<String, String>, // key: 名称, value: 节点ID
}

impl ParallelNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: ParallelConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for ParallelNode".into()))?;

        Ok(Self {
            base: NodeBase::new(id, processor),
            branches: config.branches,
        })
    }
}

#[impl_executable]
impl Executable for ParallelNode {
    async fn core_execute(
        &self,
        input: Option<FlowData>,
        context: Arc<Context>,
    ) -> Result<FlowOutput> {
        let mut set = JoinSet::new();

        for (key, node_id) in &self.branches {
            let node = context
                .get_node(node_id)
                .ok_or(Error::NodeNotFound(node_id.clone()))?
                .clone();

            let input_clone = input.clone();
            let context_clone = context.clone();
            let key = key.clone();
            let node_id = node_id.clone();

            // 使用独立的 spawn_task 方法启动任务
            set.spawn(spawn_task(node_id, key, node, input_clone, context_clone));
        }

        let mut vec = Vec::new();

        // let mut outputs = OutputData::default_parallel();
        while let Some(res) = set.join_next().await {
            if let Ok((_key, output)) = res? {
                vec.push(output);
            } else {
                // 处理错误
                return Err(Error::ExecutionError("Task execution failed".into()));
            }
        }

        Ok(vec.into())
    }
}

/// 启动并发任务，执行每个子节点
fn spawn_task(
    node_id: String,
    key: String,
    node: Arc<dyn Executable>,
    input: Option<FlowData>,
    context: Arc<Context>,
) -> impl std::future::Future<Output = Result<(String, ControlFlow)>> {
    async move {
        let result = node.execute(input, context).await;
        let value = result?; // result: Result<OutputValue, _>
        let data = value
            .into_data()
            .map_err(|e| Error::ExecutionError(e.to_string()))?; // now FlowData

        Ok((key, ControlFlow::new(&node_id, data)))
    }
}
