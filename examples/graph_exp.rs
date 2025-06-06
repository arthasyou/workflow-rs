use serde_json::json;
use workflow_rs::{
    graph::Graph,
    model::{
        DataPayload,
        data_payload::SingleData,
        node::{ControlNode, DataNode, DataProcessorMapping, Node, NodeType},
    },
    runner::Runner,
};

#[tokio::main]
async fn main() {
    // 创建一个新的图
    let mut graph = Graph::new_with_default_nodes().unwrap();

    // 定义节点 (不包含 Start 和 End)
    let nodes = vec![
        Node::new(
            "A",
            NodeType::Data(DataNode::Prompt),
            json!({ "template": "Node A Data" }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
        Node::new(
            "B",
            NodeType::Data(DataNode::Prompt),
            json!({ "template": "Node B Data" }),
            DataProcessorMapping::default(),
            Some("input1".to_string()),
            Some("output1".to_string()),
        ),
        // Node::new(
        //     "C",
        //     NodeType::Data(DataNode::Prompt),
        //     json!({ "message": "Node C Data" }),
        //     DataProcessorMapping::default(),
        //     None,
        //     None,
        // ),
        // Node::new(
        //     "D",
        //     NodeType::Data(DataNode::Prompt),
        //     json!({ "message": "Node D Data" }),
        //     DataProcessorMapping::default(),
        //     Some("input2".to_string()),
        //     Some("output2".to_string()),
        // ),
        // Node::new(
        //     "E",
        //     NodeType::Data(DataNode::Prompt),
        //     json!({ "message": "Node E Data" }),
        //     DataProcessorMapping::default(),
        //     None,
        //     None,
        // ),
        // Node::new(
        //     "Control1",
        //     NodeType::Control(ControlNode::Branch),
        //     json!({ "condition": "x > 5" }),
        //     DataProcessorMapping::default(),
        //     None,
        //     None,
        // ),
    ];

    // 添加节点
    for node in nodes {
        graph.add_node(node).unwrap();
    }

    // // 设置 Start 节点
    // graph
    //     .set_start_node(Node::new(
    //         "Start",
    //         NodeType::Data(DataNode::Prompt),
    //         json!({ "message": "Start Node" }),
    //         DataProcessorMapping::default(),
    //         None,
    //         None,
    //     ))
    //     .unwrap();

    // // 设置 End 节点
    // graph
    //     .set_end_node(Node::new(
    //         "End",
    //         NodeType::Data(DataNode::Prompt),
    //         json!({ "message": "End Node" }),
    //         DataProcessorMapping::default(),
    //         None,
    //         None,
    //     ))
    //     .unwrap();

    // 添加边
    graph.add_edge("start", "A").unwrap();
    graph.add_edge("A", "B").unwrap();
    // graph.add_edge("A", "C").unwrap();
    // graph.add_edge("B", "D").unwrap();
    // graph.add_edge("C", "E").unwrap();
    // graph.add_edge("D", "End").unwrap();
    graph.add_edge("B", "end").unwrap();
    // graph.add_edge("Control1", "B").unwrap();

    // 使用 Runner 执行图
    let mut runner = Runner::new();

    let input = SingleData::new_text("test");
    let input = DataPayload::new_single(input); // 转换为 DataPayload 类型

    let r = runner.run(&mut graph, input).await.unwrap();
    println!("Graph execution result: {:?}", r);
}
