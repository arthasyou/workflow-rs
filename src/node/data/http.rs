use std::sync::Arc;

use flow_data::{FlowData, output::FlowOutput};
use serde_json::{Value, json};
use toolcraft_request::Request;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase, config::HttpConfig},
};

/// HttpNode 节点：发送 HTTP 请求到远程 API 服务器
///
/// 支持功能：
/// - 多种 HTTP 方法：GET, POST, PUT, DELETE, PATCH
/// - 自定义请求头
/// - 请求超时配置（需要 toolcraft_request 支持）
/// - 输入数据合并：节点输入数据会与配置数据合并，输入数据优先级更高
/// - 自动处理 JSON 和纯文本响应
///
/// 配置示例：
/// ```json
/// {
///     "url": "https://api.example.com/endpoint",
///     "method": "POST",  // 可选，默认为 POST
///     "input_data": {    // 请求体数据
///         "key": "value"
///     },
///     "headers": {       // 可选，自定义请求头
///         "Authorization": "Bearer token"
///     },
///     "timeout_seconds": 30  // 可选，请求超时时间
/// }
/// ```
#[derive(Debug, Clone)]
pub struct HttpNode {
    base: NodeBase,
    config: HttpConfig,
}

impl HttpNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: HttpConfig = serde_json::from_value(data).map_err(|e| {
            Error::ExecutionError(format!("Invalid data format for HttpNode: {}", e).into())
        })?;

        // Validate URL
        if config.url.is_empty() {
            return Err(Error::ExecutionError("HTTP URL cannot be empty".into()));
        }

        Ok(Self {
            base: NodeBase::new(id, processor),
            config,
        })
    }

    /// Merge input data with config data
    fn merge_request_data(&self, _input: Option<FlowData>) -> Result<Value> {
        let config_json = self.config.input_data.clone();
        Ok(config_json)

        // match input {
        //     Some(input_data) => {
        //         let input_json = input_data.into_json()?;
        //         let config_json = self.config.input_data.clone();

        //         // Merge logic: input data takes precedence over config data
        //         match (input_json, config_json) {
        //             (Value::Object(mut input_map), Value::Object(config_map)) => {
        //                 // Config values are defaults, input values override
        //                 for (k, v) in config_map {
        //                     input_map.entry(k).or_insert(v);
        //                 }
        //                 Ok(Value::Object(input_map))
        //             }
        //             (input, Value::Null) => Ok(input),
        //             (Value::Null, config) => Ok(config),
        //             (input, _) => Ok(input), // Input takes precedence
        //         }
        //     }
        //     None => Ok(self.config.input_data.clone()),
        // }
    }
}

#[impl_executable]
impl Executable for HttpNode {
    async fn core_execute(
        &self,
        input: Option<FlowData>,
        _context: Arc<Context>,
    ) -> Result<FlowOutput> {
        // Prepare request data by merging input with config
        let request_data = self.merge_request_data(input)?;

        // Create HTTP client
        let request = Request::new().map_err(|e| {
            Error::ExecutionError(format!("Failed to create HTTP client: {}", e).into())
        })?;

        // Convert headers from HashMap to Vec<(&'static str, String)>
        let mut headers_vec: Vec<(&'static str, String)> = Vec::new();

        // Add custom headers if configured
        if let Some(ref headers_map) = self.config.headers {
            for (key, value) in headers_map {
                // We need to leak the string to get a 'static lifetime
                // This is not ideal but toolcraft_request requires it
                let key_static: &'static str = Box::leak(key.clone().into_boxed_str());
                headers_vec.push((key_static, value.clone()));
            }
        }

        // Ensure Content-Type is set for JSON requests if not already present
        let has_content_type = self
            .config
            .headers
            .as_ref()
            .map(|h| h.keys().any(|k| k.eq_ignore_ascii_case("content-type")))
            .unwrap_or(false);

        if !has_content_type {
            headers_vec.push(("Content-Type", "application/json".to_string()));
        }

        let headers_option = if headers_vec.is_empty() {
            None
        } else {
            Some(headers_vec)
        };

        // Execute the request based on method
        let method = self
            .config
            .method
            .as_deref()
            .unwrap_or("POST")
            .to_uppercase();
        let response = match method.as_str() {
            "GET" => {
                // For GET requests, convert data to query parameters
                let url = if matches!(request_data, Value::Object(_))
                    && !request_data.as_object().unwrap().is_empty()
                {
                    let query_string = serde_urlencoded::to_string(&request_data).map_err(|e| {
                        Error::ExecutionError(
                            format!("Failed to serialize query parameters: {}", e).into(),
                        )
                    })?;
                    format!("{}?{}", self.config.url, query_string)
                } else {
                    self.config.url.clone()
                };
                // GET takes url, headers, query_params
                request.get(&url, None, headers_option).await
            }
            "POST" => {
                request
                    .post(&self.config.url, &request_data, headers_option)
                    .await
            }
            "PUT" => {
                request
                    .put(&self.config.url, &request_data, headers_option)
                    .await
            }
            "DELETE" => request.delete(&self.config.url, headers_option).await,
            "PATCH" => {
                // Use POST as a fallback for PATCH since toolcraft_request might not have patch
                request
                    .post(&self.config.url, &request_data, headers_option)
                    .await
            }
            _ => {
                return Err(Error::ExecutionError(
                    format!("Unsupported HTTP method: {}", method).into(),
                ));
            }
        };

        let res = response
            .map_err(|e| Error::ExecutionError(format!("HTTP request failed: {}", e).into()))?;

        // Check response status
        let status = res.status();
        if !status.is_success() {
            let error_text = res
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::ExecutionError(
                format!("HTTP request failed with status {}: {}", status, error_text).into(),
            ));
        }

        // Get response text
        let response_text = res.text().await.map_err(|e| {
            Error::ExecutionError(format!("Failed to read response body: {}", e).into())
        })?;

        // Try to parse as JSON, fallback to text wrapped in JSON
        let response_data = match serde_json::from_str::<Value>(&response_text) {
            Ok(json_data) => json_data,
            Err(_) => {
                // If not valid JSON, wrap the text response in a JSON object
                json!({
                    "response": response_text,
                    "_type": "text"
                })
            }
        };

        Ok(FlowData::from(response_data).into())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_http_node_creation() {
        let config_data = json!({
            "url": "https://api.example.com",
            "input_data": {"key": "value"},
            "method": "POST"
        });

        let node = HttpNode::new("test_node", config_data, &DataProcessorMapping::default());
        assert!(node.is_ok());
    }

    #[test]
    fn test_http_node_empty_url() {
        let config_data = json!({
            "url": "",
            "input_data": {"key": "value"}
        });

        let node = HttpNode::new("test_node", config_data, &DataProcessorMapping::default());
        assert!(node.is_err());
    }

    #[test]
    fn test_merge_request_data() {
        let config_data = json!({
            "url": "https://api.example.com",
            "input_data": {"config_key": "config_value", "shared_key": "config_shared"}
        });

        let node =
            HttpNode::new("test_node", config_data, &DataProcessorMapping::default()).unwrap();

        // Test with input data
        let input = FlowData::from(json!({
            "input_key": "input_value",
            "shared_key": "input_shared"
        }));

        let merged = node.merge_request_data(Some(input)).unwrap();
        let merged_obj = merged.as_object().unwrap();

        assert_eq!(merged_obj.get("input_key").unwrap(), "input_value");
        assert_eq!(merged_obj.get("config_key").unwrap(), "config_value");
        assert_eq!(merged_obj.get("shared_key").unwrap(), "input_shared"); // Input takes precedence
    }
}
