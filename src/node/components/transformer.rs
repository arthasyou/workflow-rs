use serde_json::Value;

use crate::error::Result;

/// 转换节点处理逻辑
pub fn execute_transformer(config: &Option<Value>, input: &Value) -> Result<Value> {
    if let Some(cfg) = config {
        if let Some(transformation) = cfg.get("transformation").and_then(|v| v.as_str()) {
            match transformation {
                "to_uppercase()" => {
                    if let Some(s) = input.as_str() {
                        return Ok(Value::String(s.to_uppercase()));
                    }
                }
                "to_lowercase()" => {
                    if let Some(s) = input.as_str() {
                        return Ok(Value::String(s.to_lowercase()));
                    }
                }
                _ => {
                    return Ok(Value::String(format!(
                        "Unknown transformation: {}",
                        transformation
                    )));
                }
            }
        }
    }

    Ok(Value::String("Invalid Transformer config".to_string()))
}
