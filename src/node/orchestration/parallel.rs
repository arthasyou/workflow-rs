use std::{
    sync::{Arc, Mutex},
    thread,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::Result, node::Executable};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelNode {
    pub id: String,
    pub target_node_ids: Vec<String>,
    pub merge_strategy: String,
}

impl ParallelNode {
    pub fn new(id: &str, target_node_ids: Vec<String>, merge_strategy: &str) -> Self {
        Self {
            id: id.to_string(),
            target_node_ids,
            merge_strategy: merge_strategy.to_string(),
        }
    }
}

// impl Executable for ParallelNode {
//     fn execute(&self, input: Value) -> Result<Value> {
//         println!("Starting ParallelNode [{}]", self.id);

//         let shared_output = Arc::new(Mutex::new(Vec::new()));

//         let mut handles = Vec::new();

//         for node_id in &self.target_node_ids {
//             let output_clone = Arc::clone(&shared_output);
//             let node_id_clone = node_id.clone();
//             let input_clone = input.clone();

//             let handle = thread::spawn(move || {
//                 println!("Executing Node [{}] in parallel", node_id_clone);

//                 // 模拟执行逻辑，简单拼接输出
//                 let result = format!("Node [{}] executed", node_id_clone);

//                 let mut output = output_clone.lock().unwrap();
//                 output.push(result);
//             });

//             handles.push(handle);
//         }

//         for handle in handles {
//             handle.join().unwrap();
//         }

//         let final_output = shared_output.lock().unwrap().join(", ");

//         Ok(Value::String(final_output))
//     }
// }
