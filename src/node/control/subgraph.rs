use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use workflow_macro::impl_executable;

use crate::{
    error::{Error, Result},
    model::{context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGraphNode {
    pub base: NodeBase,
    pub subgraph_node_ids: Vec<String>,
    pub merge_strategy: String,
}

impl SubGraphNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: SubGraphConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for SubGraphNode".into()))?;

        Ok(Self {
            base: NodeBase::new(id, processor),
            subgraph_node_ids: config.subgraph_node_ids,
            merge_strategy: config.merge_strategy,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGraphConfig {
    pub subgraph_node_ids: Vec<String>,
    pub merge_strategy: String,
}

// #[impl_executable]
// impl Executable for SubGraphNode {
//     async fn core_execute(&self, input: Value, context: Arc<Context>) -> Result<Value> {
//         let mut results = Vec::new();

//         for node_id in &self.subgraph_node_ids {
//             let node = context
//                 .get_node(node_id)
//                 .ok_or_else(|| Error::NodeNotFound(node_id.clone()))?;

//             let result = node.execute(input.clone(), context.clone()).await?;
//             results.push(result);
//         }

//         let output = match self.merge_strategy.as_str() {
//             "concat" => Value::String(
//                 results
//                     .iter()
//                     .map(|v| v.to_string())
//                     .collect::<Vec<String>>()
//                     .join(", "),
//             ),
//             "json_merge" => Value::Array(results),
//             _ => Value::String("Unknown merge strategy".to_string()),
//         };

//         Ok(output)
//     }
// }
