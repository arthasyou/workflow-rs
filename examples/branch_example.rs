// use std::collections::HashMap;

// use serde_json::json;
// use workflow_rs::{
//     graph::Graph,
//     node::{BranchNode, PromptNode},
//     runner::Runner,
// };

// fn main() {
//     let mut graph = Graph::new();

//     // 创建 BranchNode
//     let mut branches = HashMap::new();
//     branches.insert("yes".to_string(), "accept".to_string());
//     branches.insert("no".to_string(), "reject".to_string());

//     let branch_node = BranchNode::new("branch", branches, Some("fallback".to_string()));
//     graph.add_node(branch_node);

//     // 创建 PromptNode
//     let accept_node = PromptNode::new("accept", "Accepted: {input}");
//     let reject_node = PromptNode::new("reject", "Rejected: {input}");
//     let fallback_node = PromptNode::new("fallback", "Fallback: {input}");

//     graph.add_node(accept_node);
//     graph.add_node(reject_node);
//     graph.add_node(fallback_node);

//     graph.compile().expect("Failed to compile graph");

//     for input_str in ["yes", "no", "maybe"] {
//         let input = json!(input_str);
//         let mut runner = Runner::new(&graph);
//         let result = runner.run(input).expect("Run failed");
//         println!("Input '{}' result: {}", input_str, result);
//     }
// }

fn main() {}
