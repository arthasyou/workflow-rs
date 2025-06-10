use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::DataPayload;

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
pub struct BranchPayload {
    pub condition: String,
    #[serde(rename = "nodeId")]
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchConfig {
    pub branches: Vec<BranchPayload>,
    pub default: Option<String>,
}

impl BranchConfig {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for branch in &self.branches {
            map.insert(branch.condition.clone(), branch.node_id.clone());
        }
        map
    }
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
