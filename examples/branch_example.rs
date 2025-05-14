use std::{collections::HashMap, sync::Arc};

use serde_json::json;
use workflow_rs::{
    model::{Context, node::DataProcessorMapping},
    node::{
        Executable, config::BranchConfig, control::branch::BranchNode,
        data::prompt::PromptNode,
    },
};

#[tokio::main]
async fn main() {
    let processor = DataProcessorMapping::default();

    // 构建节点实例
    let prompt1 = PromptNode::new(
        "prompt1",
        json!({"template": "This is the first prompt"}),
        &processor,
    )
    .unwrap();

    let prompt2 = PromptNode::new(
        "prompt2",
        json!({"template": "This is the second prompt"}),
        &processor,
    )
    .unwrap();

    let prompt_default = PromptNode::new(
        "default",
        json!({"template": "Default branch executed"}),
        &processor,
    )
    .unwrap();

    // 构建 BranchConfig
    let mut branches = HashMap::new();
    branches.insert("path1".to_string(), "prompt1".to_string());
    branches.insert("path2".to_string(), "prompt2".to_string());

    let branch_config = BranchConfig {
        branches,
        default: Some("default".to_string()),
    };

    // 将 BranchConfig 转换为 Value
    let branch_data = serde_json::to_value(branch_config).unwrap();

    // 构建 BranchNode
    let branch_node = BranchNode::new("branch1", branch_data, &processor).unwrap();

    // 构建 Context
    let mut nodes: HashMap<String, Arc<dyn Executable>> = HashMap::new();
    nodes.insert("prompt1".to_string(), Arc::new(prompt1));
    nodes.insert("prompt2".to_string(), Arc::new(prompt2));
    nodes.insert("default".to_string(), Arc::new(prompt_default));
    nodes.insert("branch1".to_string(), Arc::new(branch_node));

    let context = Context {
        nodes,
        metadata: HashMap::new(),
    };

    let context = Arc::new(context);

    // 执行 BranchNode，传入不同的输入进行测试
    let input1 = json!("path1");
    let input2 = json!("path2");
    let input_default = json!("unknown_path");

    let branch_node = context.get_node("branch1").unwrap();

    let result1 = branch_node.execute(input1, context.clone()).await.unwrap();
    println!("Result 1: {}", result1);

    let result2 = branch_node.execute(input2, context.clone()).await.unwrap();
    println!("Result 2: {}", result2);

    let result_default = branch_node
        .execute(input_default, context.clone())
        .await
        .unwrap();
    println!("Result Default: {}", result_default);
}
