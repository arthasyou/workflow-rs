use std::sync::Arc;

use serde_json::Value;

use crate::error::Result;

/// 通用 Processor Trait
pub trait ProcessorTrait<T> {
    fn process(&self, node_id: &str, data: &T, context: Option<&Value>) -> Result<T>;
}

/// 输入和输出 Processor 类型
pub type Processor<T> = Option<Arc<dyn ProcessorTrait<T> + Send + Sync>>;

pub type InputProc = Processor<Value>;
pub type OutputProc = Processor<Value>;
