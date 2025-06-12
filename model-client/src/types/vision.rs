use serde::{Deserialize, Serialize};

/// Prompt structure for vision models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionPrompt {
    /// URL or base64-encoded string of the image
    pub image: String,

    /// Optional prompt text accompanying the image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

/// Response structure from vision models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionResponse {
    /// Textual description or result generated from the image
    pub result: String,
}
