use serde_json::Value;

use crate::{error::Result, processor::ProcessorTrait};

/// LoggingProcessor 示例
pub struct LoggingProcessor;

impl ProcessorTrait<Value> for LoggingProcessor {
    fn process(&self, node_id: &str, data: &Value, _context: Option<&Value>) -> Result<Value> {
        println!("Node [{}] - Logging Input: {:?}", node_id, data);
        Ok(data.clone())
    }
}
