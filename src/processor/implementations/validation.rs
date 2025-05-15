use async_trait::async_trait;

use crate::{error::Result, model::DataPayload, processor::Processor};

/// ValidationProcessor 示例
pub struct ValidationProcessor;

#[async_trait]
impl Processor<DataPayload> for ValidationProcessor {
    async fn process(&self, data: &DataPayload) -> Result<DataPayload> {
        println!("Validating Output: {:?}", data);
        Ok(data.clone())
    }
}
