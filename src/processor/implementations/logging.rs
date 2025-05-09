use async_trait::async_trait;
use serde_json::Value;

use crate::{error::Result, processor::Processor};

/// LoggingProcessor 示例
pub struct LoggingProcessor;

#[async_trait]
impl Processor<Value> for LoggingProcessor {
    async fn process(
        &self,
        node_id: &str,
        data: &Value,
        _context: Option<&Value>,
    ) -> Result<Value> {
        println!("Node [{}] - Logging Input: {:?}", node_id, data);
        Ok(data.clone())
    }
}
