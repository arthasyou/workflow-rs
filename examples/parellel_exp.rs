use std::{collections::HashMap, sync::Arc};

use serde_json::json;
use workflow_rs::{
    model::{Context, node::DataProcessorMapping},
    node::{
        Executable, config::ParallelConfig, orchestration::ParallelNode, task::prompt::PromptNode,
    },
};

#[tokio::main]
async fn main() {
    let processor = DataProcessorMapping::default();

    // 构建 Prompt 节点
    let prompt1 = PromptNode::new(
        "prompt1",
        json!({"template": "Task 1 executed"}),
        &processor,
    )
    .unwrap();
    let prompt2 = PromptNode::new(
        "prompt2",
        json!({"template": "Task 2 executed"}),
        &processor,
    )
    .unwrap();
    let prompt3 = PromptNode::new(
        "prompt3",
        json!({"template": "Task 3 executed"}),
        &processor,
    )
    .unwrap();

    // 构建 ParallelConfig
    let mut branches = HashMap::new();
    branches.insert("task1".to_string(), "prompt1".to_string());
    branches.insert("task2".to_string(), "prompt2".to_string());
    branches.insert("task3".to_string(), "prompt3".to_string());

    let parallel_config = ParallelConfig { branches };

    // 转换为 Value
    let parallel_data = serde_json::to_value(parallel_config).unwrap();

    // 构建 ParallelNode
    let parallel_node = ParallelNode::new("parallel1", parallel_data, &processor).unwrap();

    // 构建 Context
    let mut nodes: HashMap<String, Arc<dyn Executable>> = HashMap::new();
    nodes.insert("prompt1".to_string(), Arc::new(prompt1));
    nodes.insert("prompt2".to_string(), Arc::new(prompt2));
    nodes.insert("prompt3".to_string(), Arc::new(prompt3));
    nodes.insert("parallel1".to_string(), Arc::new(parallel_node));

    let context = Arc::new(Context {
        nodes,
        metadata: HashMap::new(),
    });

    // 执行 ParallelNode
    let input = json!("Parallel Execution");
    let parallel_node = context.get_node("parallel1").unwrap();
    let result = parallel_node.execute(input, context.clone()).await.unwrap();

    println!("ParallelNode Result: {}", result);
}
