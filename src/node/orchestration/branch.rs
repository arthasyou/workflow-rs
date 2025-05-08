use std::collections::HashMap;

use serde_json::Value;
use workflow_macros::impl_executable;

use crate::{
    error::{Error, Result},
    node::{Executable, NodeBase},
};

#[derive(Debug, Clone)]
pub struct BranchNode {
    pub base: NodeBase,
    pub branches: HashMap<String, String>,
    pub default: Option<String>,
}

impl BranchNode {
    pub fn new(id: &str, branches: HashMap<String, String>, default: Option<String>) -> Self {
        let base = NodeBase::new(id);
        Self {
            base,
            branches,
            default,
        }
    }
}

#[impl_executable]
impl Executable for BranchNode {
    fn core_execute(&self, input: Value) -> Result<Value> {
        let input_str = input.as_str().ok_or(Error::InvalidBranchInput)?;

        if let Some(target) = self.branches.get(input_str) {
            return Ok(Value::String(target.clone()));
        }

        if let Some(default) = &self.default {
            return Ok(Value::String(default.clone()));
        }

        Err(Error::BranchConfigMissing)
    }

    // fn get_base(&self) -> &NodeBase {
    //     &self.base
    // }

    // fn clone_box(&self) -> Box<dyn Executable> {
    //     Box::new(self.clone())
    // }
}
