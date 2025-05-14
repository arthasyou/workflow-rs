use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use workflow_macro::impl_executable;

use crate::{
    error::{Error, Result},
    model::{context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase, config::AggregatorConfig},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatorNode {
    pub base: NodeBase,
    pub branches: HashMap<String, String>, // key: 名称, value: 节点ID
}

impl AggregatorNode {
    pub fn new(id: &str, data: Value, processor: &DataProcessorMapping) -> Result<Self> {
        let config: AggregatorConfig = serde_json::from_value(data)
            .map_err(|_| Error::ExecutionError("Invalid data format for AggregatorNode".into()))?;

        Ok(Self {
            base: NodeBase::new(id, processor),
            branches: config.branches,
        })
    }
}

#[impl_executable]
impl Executable for AggregatorNode {
    async fn core_execute(&self, _input: Value, context: Arc<Context>) -> Result<Value> {
        let mut aggregated = serde_json::Map::new();

        for (key, node_id) in &self.branches {
            let node = context
                .get_node(node_id)
                .ok_or(Error::NodeNotFound(node_id.clone()))?
                .clone();

            let output = node.execute(json!(null), context.clone()).await?;
            aggregated.insert(key.clone(), output);
        }

        Ok(Value::Object(aggregated))
    }
}
