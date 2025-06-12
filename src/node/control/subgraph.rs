use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    graph::Graph,
    model::{DataPayload, OutputData, context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase},
    runner::Runner,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGraphNode {
    pub base: NodeBase,
    pub subgraph: Graph,
}

impl SubGraphNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: SubGraphConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for SubGraphNode".into()))?;

        Ok(Self {
            base: NodeBase::new(id, processor),
            subgraph: config.subgraph,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGraphConfig {
    pub subgraph: Graph,
    pub merge_strategy: String,
}

// #[impl_executable]
// impl Executable for SubGraphNode {
//     async fn core_execute(&self, input: DataPayload, _context: Arc<Context>) ->
// Result<OutputData> {         let mut runner = Runner::new();
//         let start_node =
//             self.subgraph.start_node.as_ref().ok_or_else(|| {
//                 Error::ExecutionError("SubGraph start node is not defined".into())
//             })?;

//         runner.set_input(start_node, input.clone());
//         let mut subgraph = self.subgraph.clone();
//         runner.run(&mut subgraph, input).await?;

//         let end_node = self
//             .subgraph
//             .end_node
//             .as_ref()
//             .ok_or_else(|| Error::ExecutionError("SubGraph end node is not defined".into()))?;

//         let output = runner.get_output(end_node)?;

//         Ok(output.clone())
//     }
// }
