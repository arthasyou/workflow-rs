use async_trait::async_trait;
use flow_data::FlowData;

use crate::processor::Processor;

/// ValidationProcessor 示例
pub struct ValidationProcessor;

#[async_trait]
impl Processor<FlowData> for ValidationProcessor {
    async fn process(&self, data: FlowData) -> Option<FlowData> {
        println!("Validating Output: {:?}", data);
        Some(data)
    }
}
