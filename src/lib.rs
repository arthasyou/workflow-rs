// use graph::Graph;
// use model::Context;
// use runner::Runner;
// use serde_json::Value;

pub mod edge;
pub mod graph;
pub mod inputs;
pub mod mcp;
pub mod model;
pub mod node;
pub mod processor;
pub mod runner;
pub mod storage;
pub mod types;

use flow_data::FlowData;

use crate::{graph::Graph, runner::Runner};

/// Workflow 模块：封装 Graph → Context → Runner 执行链路
pub struct Workflow;
use workflow_error::Result;

impl Workflow {
    /// 启动工作流：根据 Graph 生成 Context，并执行 Runner
    pub async fn start(mut graph: Graph) -> Result<FlowData> {
        // 构建 Runner 并执行
        let mut runner = Runner::new();
        runner.run(None, &mut graph, None).await
    }
}
