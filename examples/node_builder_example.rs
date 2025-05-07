use std::{any::Any, sync::Arc};

use serde_json::{Value, json};
use workflow_rs::{
    error::Result,
    node::{
        NodeBuilder, NodeKind,
        processor::{InputProcessor, OutputProcessor},
    },
    types::Executable,
};

struct LoggerProcessor;

impl InputProcessor<Value> for LoggerProcessor {
    fn process_input(
        &self,
        node_id: &str,
        input: &Value,
        _state: Option<&Arc<dyn Any + Send + Sync>>,
    ) -> Result<Value> {
        println!("Input for node {}: {:?}", node_id, input);
        Ok(input.clone())
    }
}

impl OutputProcessor<Value> for LoggerProcessor {
    fn process_output(
        &self,
        node_id: &str,
        output: &Value,
        _state: Option<&Arc<dyn Any + Send + Sync>>,
    ) -> Result<Value> {
        println!("Output for node {}: {:?}", node_id, output);
        Ok(output.clone())
    }
}

fn main() {
    let processor = Arc::new(LoggerProcessor);

    let node = NodeBuilder::new("node1", NodeKind::Prompt)
        .with_config(json!({ "template": "Hello, {input}!" }))
        .with_input_processor(Some(processor.clone()))
        .with_output_processor(Some(processor))
        .build();

    let input = json!("World");
    let result = node.execute(input);

    println!("Execution result: {:?}", result);
}
