use std::{collections::HashMap, sync::Arc};

use flow_data::{FlowData, output::FlowOutput};
use serde_json::Value;
use workflow_error::{Error, Result};
use workflow_macro::impl_executable;

use crate::{
    model::{context::Context, node::DataProcessorMapping},
    node::{Executable, NodeBase, config::AggregatorConfig},
};

#[derive(Debug, Clone)]
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
    async fn core_execute(
        &self,
        input: Option<FlowData>,
        context: Arc<Context>,
    ) -> Result<FlowOutput> {
        let mut aggregated = FlowData::new_collection();

        for (_key, node_id) in &self.branches {
            let node = context
                .get_node(node_id)
                .ok_or(Error::NodeNotFound(node_id.clone()))?
                .clone();

            let output = node.execute(input.clone(), context.clone()).await?;
            aggregated.merge_mut(output.into_data()?);
        }

        Ok(aggregated.into())
    }
}
