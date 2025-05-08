use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::Result, node::Executable};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatNode {
    pub id: String,
    pub max_repeats: usize,
    pub target_node_id: String,
}

impl RepeatNode {
    pub fn new(id: &str, max_repeats: usize, target_node_id: &str) -> Self {
        Self {
            id: id.to_string(),
            max_repeats,
            target_node_id: target_node_id.to_string(),
        }
    }
}

// impl Executable for RepeatNode {
//     fn execute(&self, input: Value) -> Result<Value> {
//         let mut output = input.clone();
//         let mut count = 0;

//         println!("Starting RepeatNode [{}]", self.id);

//         while count < self.max_repeats {
//             println!("RepeatNode [{}] - Execution #{}", self.id, count + 1);

//             // 模拟执行目标节点（这里简单返回处理次数）
//             output = Value::String(format!("Executed {} times", count + 1));

//             count += 1;
//         }

//         println!("RepeatNode [{}] completed", self.id);

//         Ok(output)
//     }
// }
