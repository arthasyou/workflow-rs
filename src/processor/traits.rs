use std::sync::Arc;

use serde_json::Value;

use crate::error::Result;

/// 通用 Processor Trait
pub trait Processor<T> {
    fn process(&self, node_id: &str, data: &T, context: Option<&Value>) -> Result<T>;
}

/// 输入和输出 Processor 类型
pub type InputProcessor = Option<Arc<dyn Processor<Value> + Send + Sync>>;
pub type OutputProcessor = Option<Arc<dyn Processor<Value> + Send + Sync>>;
