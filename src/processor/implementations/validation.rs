use serde_json::Value;

use crate::{error::Result, processor::Processor};

/// ValidationProcessor 示例
pub struct ValidationProcessor;

impl Processor<Value> for ValidationProcessor {
    fn process(&self, node_id: &str, data: &Value, _context: Option<&Value>) -> Result<Value> {
        println!("Node [{}] - Validating Output: {:?}", node_id, data);
        Ok(data.clone())
    }
}
