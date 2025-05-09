use serde_json::Value;

use crate::{error::Result, processor::Processor};

/// LoggingProcessor 示例
pub struct LoggingProcessor;

impl Processor<Value> for LoggingProcessor {
    fn process(&self, node_id: &str, data: &Value, _context: Option<&Value>) -> Result<Value> {
        println!("Node [{}] - Logging Input: {:?}", node_id, data);
        Ok(data.clone())
    }
}
