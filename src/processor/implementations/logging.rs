use async_trait::async_trait;

use crate::{error::Result, model::DataPayload, processor::Processor};

/// LoggingProcessor 示例
pub struct LoggingProcessor;

#[async_trait]
impl Processor<DataPayload> for LoggingProcessor {
    async fn process(&self, data: &DataPayload) -> Option<DataPayload> {
        println!("Logging Input: {:?}", data);
        Some(data.clone())
    }
}
