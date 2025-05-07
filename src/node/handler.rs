use std::{any::Any, sync::Arc};

use crate::error::Result;

/// 可选上下文状态对象
pub type State = Arc<dyn Any + Send + Sync>;

/// 执行前处理器：可修改输入
pub trait PreHandler<I>: Send + Sync {
    fn before_execute(&self, node_id: &str, input: &I, state: Option<&State>) -> Result<I>;
}

/// 执行后处理器：可修改输出
pub trait PostHandler<O>: Send + Sync {
    fn after_execute(&self, node_id: &str, output: &O, state: Option<&State>) -> Result<O>;
}
