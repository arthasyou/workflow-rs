// use serde_json::json;
// use workflow_rs::{
//     node::orchestration::{ParallelNode, SubGraphNode},
//     types::Executable,
// };

// fn main() {
//     let input_data = json!({"message": "Hello, Orchestration"});

//     // 测试 ParallelNode
//     let parallel_node = ParallelNode::new(
//         "parallel_1",
//         vec![
//             "node1".to_string(),
//             "node2".to_string(),
//             "node3".to_string(),
//         ],
//         "merge",
//     );
//     let parallel_output = parallel_node.execute(input_data.clone()).unwrap();
//     println!("ParallelNode Output: {:?}", parallel_output);

//     // 测试 SubGraphNode
//     let subgraph_node = SubGraphNode::new(
//         "subgraph_1",
//         vec!["sub1".to_string(), "sub2".to_string()],
//         "concat",
//     );
//     let subgraph_output = subgraph_node.execute(input_data.clone()).unwrap();
//     println!("SubGraphNode Output: {:?}", subgraph_output);
// }

fn main() {}
