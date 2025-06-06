use std::sync::Arc;

use async_trait::async_trait;

use crate::model::{DataPayload, OutputData};

#[async_trait]
pub trait Processor<T> {
    async fn process(&self, data: &T) -> Option<T>;
}

/// 输入和输出 Processor 类型
pub type InputProcessor = Option<Arc<dyn Processor<DataPayload> + Send + Sync>>;
pub type OutputProcessor = Option<Arc<dyn Processor<OutputData> + Send + Sync>>;
