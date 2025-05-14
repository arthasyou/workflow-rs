use async_trait::async_trait;
use serde_json::Value;

use crate::{error::Result, processor::Processor};

/// LoggingProcessor 示例
pub struct LoggingProcessor;

#[async_trait]
impl Processor<Value> for LoggingProcessor {
    async fn process(&self, data: &Value) -> Result<Value> {
        println!("Logging Input: {:?}", data);
        Ok(data.clone())
    }
}
