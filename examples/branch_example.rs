use serde_json::json;
use workflow_rs::{
    graph::Graph,
    node::{Node, NodeKind},
    runner::Runner,
};

fn main() {
    // 1. 创建 Graph
    let mut graph = Graph::new();

    graph.add_node(Node {
        id: "branch".to_string(),
        kind: NodeKind::Branch,
        config: Some(json!({
            "branches": {
                "yes": "accept",
                "no": "reject"
            },
            "default": "fallback"
        })),
    });

    graph.add_node(Node {
        id: "accept".to_string(),
        kind: NodeKind::Prompt,
        config: Some(json!({
            "template": "Accepted: {input}"
        })),
    });

    graph.add_node(Node {
        id: "reject".to_string(),
        kind: NodeKind::Prompt,
        config: Some(json!({
            "template": "Rejected: {input}"
        })),
    });

    // graph.add_node(Node {
    //     id: "fallback".to_string(),
    //     kind: NodeKind::Prompt,
    //     config: Some(json!({
    //         "template": "Fallback: {input}"
    //     })),
    // });

    // 注意：不需要 add_edge，Branch 自己控制流转！

    graph.compile().expect("Failed to compile graph");

    // println!("Graph compiled successfully!");
    // println!("Graph nodes: {:#?}", graph);

    // 2. 针对每个输入创建新的 Runner，确保环境干净
    // for input_str in ["yes", "no", "maybe"] {
    for input_str in ["yes"] {
        let input = json!(input_str);
        let mut runner = Runner::new(&graph); // 每次新的 Runner
        let result = runner.run(input).expect("Run failed");
        println!("Input '{}' result: {}", input_str, result);
    }
}
