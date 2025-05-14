use async_trait::async_trait;
use serde_json::Value;

use crate::{error::Result, processor::Processor};

/// ValidationProcessor 示例
pub struct ValidationProcessor;

#[async_trait]
impl Processor<Value> for ValidationProcessor {
    async fn process(&self, data: &Value) -> Result<Value> {
        println!("Validating Output: {:?}", data);
        Ok(data.clone())
    }
}
