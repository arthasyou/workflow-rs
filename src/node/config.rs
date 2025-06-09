use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    model::DataPayload,
    utils::serde_util::{deserialize_from_list, serialize_as_list},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub input: DataPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromptConfig {
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_type: String,
    pub parameters: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchConfig {
    #[serde(
        serialize_with = "serialize_as_list",
        deserialize_with = "deserialize_from_list"
    )]
    pub branches: HashMap<String, String>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelConfig {
    pub branches: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatorConfig {
    pub branches: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatConfig {
    pub child_id: String,
    pub max_iterations: usize,
}
