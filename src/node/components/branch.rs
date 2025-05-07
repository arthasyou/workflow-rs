use serde_json::Value;

use crate::error::{Error, Result};

pub fn execute_branch(config: &Option<Value>, input: &Value) -> Result<Value> {
    let input_str = input.as_str().ok_or(Error::InvalidBranchInput)?;

    if let Some(cfg) = config {
        if let Some(branches) = cfg.get("branches").and_then(|v| v.as_object()) {
            if let Some(target) = branches.get(input_str) {
                return Ok(Value::String(target.as_str().unwrap_or("").to_string()));
            }
        }

        if let Some(default) = cfg.get("default").and_then(|v| v.as_str()) {
            return Ok(Value::String(default.to_string()));
        }
    }

    Err(Error::BranchConfigMissing)
}
