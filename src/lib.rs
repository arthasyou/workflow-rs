// use graph::Graph;
// use model::Context;
// use runner::Runner;
// use serde_json::Value;

pub mod edge;
pub mod error;
pub mod graph;
pub mod inputs;
pub mod model;
pub mod node;
pub mod processor;
pub mod runner;
pub mod storage;
pub mod utils;

use model::DataPayload;

use crate::{error::Result, graph::Graph, runner::Runner};

/// Workflow 模块：封装 Graph → Context → Runner 执行链路
pub struct Workflow;

impl Workflow {
    /// 启动工作流：根据 Graph 生成 Context，并执行 Runner
    pub async fn start(mut graph: Graph, input: DataPayload) -> Result<()> {
        // 构建 Runner 并执行
        let mut runner = Runner::new();
        runner.run(&mut graph, input).await
    }
}
