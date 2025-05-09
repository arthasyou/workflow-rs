use serde_json::Value;
use workflow_macro::impl_executable;

use crate::{
    error::Result,
    node::{Executable, NodeBase},
};

#[derive(Debug, Clone)]
pub struct RepeatNode {
    pub base: NodeBase,
    pub child_id: String,
    pub max_iterations: usize,
}

impl RepeatNode {
    pub fn new(id: &str, child_id: &str, max_iterations: usize) -> Self {
        Self {
            base: NodeBase::new(id),
            child_id: child_id.to_string(),
            max_iterations,
        }
    }
}

#[impl_executable]
impl Executable for RepeatNode {
    fn core_execute(&self, _input: Value) -> Result<Value> {
        Ok(Value::String("RepeatNode executed".to_string()))
    }
}
