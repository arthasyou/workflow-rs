use serde_json::Value;
use workflow_macro::impl_executable;

use crate::{
    error::Result,
    node::{Executable, NodeBase},
};

#[derive(Debug, Clone)]
pub struct ParallelNode {
    pub base: NodeBase,
    pub child_ids: Vec<String>,
}

impl ParallelNode {
    pub fn new(id: &str, child_ids: Vec<String>) -> Self {
        Self {
            base: NodeBase::new(id),
            child_ids,
        }
    }
}

// #[impl_executable]
// impl Executable for ParallelNode {
//     fn core_execute(&self, input: Value) -> Result<Value> {
//         let mut results = Vec::new();
//         let mut success_count = 0;
//         let mut error_count = 0;

//         for child_id in &self.child_ids {
//             // 这里假设通过全局方法获取节点实例
//             if let Some(child_node) = crate::graph::get_node(child_id) {
//                 match child_node.execute(input.clone()) {
//                     Ok(output) => {
//                         results.push(json!({"id": child_id, "output": output}));
//                         success_count += 1;
//                     }
//                     Err(err) => {
//                         results.push(json!({"id": child_id, "error": err.to_string()}));
//                         error_count += 1;
//                     }
//                 }
//             } else {
//                 results.push(json!({"id": child_id, "error": "Node not found"}));
//                 error_count += 1;
//             }
//         }

//         let status = if error_count == 0 {
//             "success"
//         } else if success_count == 0 {
//             "failed"
//         } else {
//             "partial_success"
//         };

//         Ok(json!({
//             "status": status,
//             "results": results
//         }))
//     }
// }
