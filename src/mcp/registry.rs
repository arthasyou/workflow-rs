use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use mcp_client::client::McpClient;
use mcp_transport::client::{impls::sse::SseTransport, traits::ClientTransport};
use once_cell::sync::Lazy;
use workflow_error::Result;

/// MCP Client Registry
#[derive(Default)]
pub struct McpClientRegistry {
    clients: Mutex<HashMap<String, Arc<McpClient<SseTransport>>>>,
}

impl McpClientRegistry {
    pub fn register(&self, server_id: &str, client: Arc<McpClient<SseTransport>>) {
        let mut map = self.clients.lock().unwrap();
        map.insert(server_id.to_string(), client);
    }

    pub fn get(&self, server_id: &str) -> workflow_error::Result<Arc<McpClient<SseTransport>>> {
        let map = self.clients.lock().unwrap();
        map.get(server_id).cloned().ok_or_else(|| {
            workflow_error::Error::ExecutionError(format!(
                "MCP client not found for server_id: {}",
                server_id
            ))
        })
    }
}

/// Global MCP client registry
pub static MCP_CLIENT_REGISTRY: Lazy<McpClientRegistry> = Lazy::new(McpClientRegistry::default);

/// Initialize and register a default MCP client
pub async fn register_mcp_clients(configs: Vec<(&str, &str)>) -> Result<()> {
    for (server_id, url) in configs {
        let transport = SseTransport::new(url);
        transport.start().await?;
        let client = Arc::new(McpClient::new(transport));
        MCP_CLIENT_REGISTRY.register(server_id, client);
    }
    Ok(())
}
