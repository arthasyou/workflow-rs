use serde_json::{Map, Value};

use crate::error::Result;

/// 聚合节点处理逻辑
pub fn execute_aggregator(config: &Option<Value>, input: &Value) -> Result<Value> {
    if let Some(cfg) = config {
        if let Some(sources) = cfg.get("sources").and_then(|v| v.as_array()) {
            if let Some(method) = cfg.get("method").and_then(|v| v.as_str()) {
                match method {
                    "merge" => {
                        let mut merged = Map::new();
                        for source in sources {
                            if let Some(key) = source.as_str() {
                                if let Some(value) = input.get(key) {
                                    merged.insert(key.to_string(), value.clone());
                                }
                            }
                        }
                        return Ok(Value::Object(merged));
                    }
                    "sum" => {
                        let mut sum = 0;
                        for source in sources {
                            if let Some(key) = source.as_str() {
                                if let Some(value) = input.get(key) {
                                    if let Some(num) = value.as_i64() {
                                        sum += num;
                                    }
                                }
                            }
                        }
                        return Ok(Value::Number(sum.into()));
                    }
                    _ => return Ok(Value::String(format!("Unknown method: {}", method))),
                }
            }
        }
    }

    Ok(Value::String("Invalid Aggregator config".to_string()))
}
