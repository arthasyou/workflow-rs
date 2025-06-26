use std::sync::Arc;

use flow_data::{FlowData, output::FlowOutput};
use mcp_client::client::McpClient;
use mcp_core::protocol::message::JsonRpcRequest;
use mcp_transport::client::impls::sse::SseTransport;
use serde::Deserialize;
use serde_json::{Value, json};
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    mcp::registry::MCP_CLIENT_REGISTRY,
    model::{context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase},
};

#[derive(Debug, Clone, Deserialize)]
struct McpNodeConfig {
    server_id: String,
    call_name: String,
    input: Value,
}

#[derive(Clone)]
pub struct McpNode {
    base: NodeBase,
    client: Arc<McpClient<SseTransport>>,
    call_name: String,
    input_data: FlowData,
}

impl std::fmt::Debug for McpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("McpNode")
            .field("base", &self.base)
            .field("call_name", &self.call_name)
            .field("input_data", &self.input_data)
            .finish()
    }
}

impl McpNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: McpNodeConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for McpNode".into()))?;

        let client = MCP_CLIENT_REGISTRY.get(&config.server_id)?;

        Ok(Self {
            base: NodeBase::new(id, processor),
            client,
            call_name: config.call_name,
            input_data: FlowData::from(config.input),
        })
    }
}

#[impl_executable]
impl Executable for McpNode {
    async fn core_execute(
        &self,
        _input: Option<FlowData>,
        _context: Arc<Context>,
    ) -> Result<FlowOutput> {
        let req = JsonRpcRequest::new(
            Some(1),
            "tools/call",
            Some(json!({
                "name": self.call_name,
                "arguments": "".to_string(),
            })),
        );

        let resp = self.client.send_resquest(req).await?;
        let result = FlowData::try_from_json(resp)?;

        Ok(result.into())
    }
}
