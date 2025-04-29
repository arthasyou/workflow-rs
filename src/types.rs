use serde_json::Value;

use crate::error::Result;

pub trait Executable {
    fn execute(&self, input: Value) -> Result<Value>;
}
