use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::Result, node::Executable};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGraphNode {
    pub id: String,
    pub subgraph_node_ids: Vec<String>,
    pub merge_strategy: String,
}

impl SubGraphNode {
    pub fn new(id: &str, subgraph_node_ids: Vec<String>, merge_strategy: &str) -> Self {
        Self {
            id: id.to_string(),
            subgraph_node_ids,
            merge_strategy: merge_strategy.to_string(),
        }
    }
}

// impl Executable for SubGraphNode {
//     fn execute(&self, input: Value) -> Result<Value> {
//         println!("Executing SubGraphNode [{}]", self.id);

//         let mut results = Vec::new();

//         for node_id in &self.subgraph_node_ids {
//             println!("Executing SubNode [{}]", node_id);

//             let result = format!("SubNode [{}] output", node_id);
//             results.push(result);
//         }

//         let final_output = results.join(", ");

//         Ok(Value::String(final_output))
//     }
// }
