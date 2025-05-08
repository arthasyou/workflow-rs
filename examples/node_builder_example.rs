// use serde_json::json;
// use workflow_rs::node::{NodeKind, builder::NodeBuilder, config::*};

// fn main() {
//     // Prompt Node
//     let prompt_config = PromptConfig {
//         template: "Hello, {input}!".to_string(),
//     };

//     let node1 = NodeBuilder::new("node1", NodeKind::Prompt)
//         .with_prompt_config(prompt_config)
//         .build()
//         .unwrap();

//     println!("Node 1: {:?}", node1);

//     // Model Node
//     let model_config = ModelConfig {
//         model_type: "gpt".to_string(),
//         parameters: json!({ "temperature": 0.7 }),
//     };

//     let node2 = NodeBuilder::new("node2", NodeKind::Model)
//         .with_model_config(model_config)
//         .build()
//         .unwrap();

//     println!("Node 2: {:?}", node2);

//     // Branch Node
//     let branch_config = BranchConfig {
//         condition: "input == 'yes'".to_string(),
//         target_nodes: vec!["node3".to_string(), "node4".to_string()],
//     };

//     let node3 = NodeBuilder::new("node3", NodeKind::Branch)
//         .with_branch_config(branch_config)
//         .build()
//         .unwrap();

//     println!("Node 3: {:?}", node3);
// }

fn main() {}
