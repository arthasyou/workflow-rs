use std::io;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Graph not compiled.")]
    GraphNotCompiled,

    #[error("Node `{0}` already exists.")]
    NodeAlreadyExists(Box<str>),

    #[error("Node `{0}` not found.")]
    NodeNotFound(Box<str>),

    #[error("Execution error: {0}")]
    ExecutionError(Box<str>),

    #[error("Invalid edge from `{start}` to `{end}`.")]
    InvalidEdge { start: Box<str>, end: Box<str> },

    #[error("No end node found.")]
    NoEndNode,

    #[error("Cycle detected in graph.")]
    CycleDetected,

    #[error("Invalid branch input.")]
    InvalidBranchInput,

    #[error("Node config missing.")]
    NodeConfigMissing,

    #[error("Join task error: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("toolcraft error: {0}")]
    ToolcraftError(#[from] toolcraft_request::error::Error),

    // #[error("config error: {0}")]
    // ServiceError(#[from] service_utils_rs::error::Error),
    #[error("Mismatched flow data type")]
    FlowTypeMismatch,

    #[error("Stream chunk error: {0}")]
    StreamChunkError(Box<str>),

    #[error("mcp error: {0}")]
    McpError(#[from] mcp_error::Error),

    #[error("model error: {0}")]
    ModelError(#[from] model_gateway_rs::error::Error),

    #[error("Other system error: {0}")]
    SystemError(Box<str>),

    #[error("Unknown boxed error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub type Result<T> = core::result::Result<T, Error>;
