use serde_json::json;
use workflow_rs::{
    graph::Graph,
    model::node::{ControlNode, DataNode, DataProcessorMapping, Node, NodeType},
};

fn main() {
    // 创建一个新的图
    let mut graph = Graph::new();

    // 定义节点 (不包含 Start 和 End)
    let nodes = vec![
        Node::new(
            "A",
            NodeType::Data(DataNode::Prompt),
            json!({ "message": "Node A Data" }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
        Node::new(
            "B",
            NodeType::Data(DataNode::Prompt),
            json!({ "message": "Node B Data" }),
            DataProcessorMapping::default(),
            Some("input1".to_string()),
            Some("output1".to_string()),
        ),
        Node::new(
            "C",
            NodeType::Data(DataNode::Prompt),
            json!({ "message": "Node C Data" }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
        Node::new(
            "D",
            NodeType::Data(DataNode::Prompt),
            json!({ "message": "Node D Data" }),
            DataProcessorMapping::default(),
            Some("input2".to_string()),
            Some("output2".to_string()),
        ),
        Node::new(
            "E",
            NodeType::Data(DataNode::Prompt),
            json!({ "message": "Node E Data" }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
        Node::new(
            "Control1",
            NodeType::Control(ControlNode::Branch),
            json!({ "condition": "x > 5" }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
    ];

    // 添加节点
    for node in nodes {
        graph.add_node(node).unwrap();
    }

    // 设置 Start 节点
    graph
        .set_start_node(Node::new(
            "Start",
            NodeType::Data(DataNode::Prompt),
            json!({ "message": "Start Node" }),
            DataProcessorMapping::default(),
            None,
            None,
        ))
        .unwrap();

    // 设置 End 节点
    graph
        .set_end_node(Node::new(
            "End",
            NodeType::Data(DataNode::Prompt),
            json!({ "message": "End Node" }),
            DataProcessorMapping::default(),
            None,
            None,
        ))
        .unwrap();

    // 添加边
    graph.add_edge("Start", "A").unwrap();
    graph.add_edge("A", "B").unwrap();
    graph.add_edge("A", "C").unwrap();
    graph.add_edge("B", "D").unwrap();
    graph.add_edge("C", "E").unwrap();
    graph.add_edge("D", "End").unwrap();
    graph.add_edge("E", "End").unwrap();
    graph.add_edge("Control1", "B").unwrap();

    // 编译图
    match graph.compile() {
        Ok(_) => {
            println!("Graph compiled successfully.");
            println!("Predecessors: {:#?}", graph.predecessors);
            println!("Successors: {:#?}", graph.successors);
        }
        Err(e) => println!("Compile Error: {:?}", e),
    }

    // JSON 序列化
    let json = graph.to_json();
    // println!("Serialized Graph JSON:\n{}", json);

    // 反序列化
    match Graph::from_json(&json) {
        Ok(_) => {
            // println!("Deserialization successful.")
        }
        Err(e) => println!("Deserialization Error: {:?}", e),
    }
}
