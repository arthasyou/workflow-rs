use serde_json::{Value, json};
use workflow_rs::{
    Workflow,
    graph::Graph,
    model::node::{ControlNode, DataNode, DataProcessorMapping, Node, NodeType},
};

#[tokio::main]
async fn main() {
    // 创建一个新的图
    let mut graph = Graph::new();

    // 定义节点 (不包含 Start 和 End)
    let nodes = vec![
        // Node::new(
        //     "A",
        //     NodeType::Data(DataNode::Prompt),
        //     json!({ "template": "Node A Data" }),
        //     DataProcessorMapping::default(),
        //     None,
        //     None,
        // ),
        // Node::new(
        //     "B",
        //     NodeType::Data(DataNode::Prompt),
        //     json!({ "template": "Node B Data" }),
        //     DataProcessorMapping::default(),
        //     Some("input1".to_string()),
        //     Some("output1".to_string()),
        // ),
        // Node::new(
        //     "C",
        //     NodeType::Data(DataNode::Prompt),
        //     json!({ "template": "Node C Data" }),
        //     DataProcessorMapping::default(),
        //     None,
        //     None,
        // ),
        // Node::new(
        //     "D",
        //     NodeType::Data(DataNode::Identity),
        //     json!({}),
        //     DataProcessorMapping::default(),
        //     None,
        //     None,
        // ),
        // Node::new(
        //     "Control1",
        //     NodeType::Control(ControlNode::Branch),
        //     json!({
        //         "branches": [
        //             { "condition": "A", "nodeId": "A" },
        //             { "condition": "B", "nodeId": "B" }
        //         ],
        //         "default": "C"
        //     }),
        //     DataProcessorMapping::default(),
        //     None,
        //     None,
        // ),
        Node::new(
            "start",
            NodeType::Data(DataNode::Input),
            serde_json::json!({
                "input": {
                        "type": "Single",
                        "value": {
                            "type": "Text",
                            "value": "Hello, this is a test input for the LLM node."
                        }
                    }
            }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
        Node::new(
            "end",
            NodeType::Data(DataNode::Identity),
            Value::Null,
            DataProcessorMapping::default(),
            None,
            None,
        ),
        Node::new(
            "llm",
            NodeType::Data(DataNode::LLM),
            json!({
                "base_url": "http://localhost:11434/v1",
                "model": "llama3.2",
                "api_key": "your_api"
            }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
    ];

    // 添加节点
    for node in nodes {
        graph.add_node(node).unwrap();
    }

    // 添加边
    graph.add_edge("start", "llm", None, None).unwrap();
    graph.add_edge("llm", "end", None, None).unwrap();

    // graph.add_edge("Control1", "A").unwrap();
    // graph.add_edge("Control1", "B").unwrap();
    // graph.add_edge("Control1", "C").unwrap();

    // graph.add_edge("A", "D").unwrap();
    // graph.add_edge("B", "D").unwrap();
    // graph.add_edge("C", "D").unwrap();
    // graph.add_edge("D", "end").unwrap();

    let graph_json = graph.to_json();
    // println!("Graph JSON: {}", graph_json);

    let ggg = Graph::from_json(&graph_json).unwrap();
    // println!("Graph from JSON: {:#?}", ggg);

    let r = Workflow::start(ggg).await.unwrap();
    println!("Graph execution result: {:?}", r);
}
