use async_trait::async_trait;
use flow_data::FlowData;

use crate::processor::Processor;

/// LoggingProcessor 示例
pub struct LoggingProcessor;

#[async_trait]
impl Processor<FlowData> for LoggingProcessor {
    async fn process(&self, data: FlowData) -> Option<FlowData> {
        println!("Logging Input: {:?}", data);
        Some(data)
    }
}
