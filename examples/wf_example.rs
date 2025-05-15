use serde_json::json;
use workflow_rs::{
    Workflow,
    graph::Graph,
    model::node::{DataNode, DataProcessorMapping, Node, NodeType},
};

#[tokio::main]
async fn main() {
    // 构建节点
    let node1 = Node::new(
        "node1",
        NodeType::Data(DataNode::Prompt),
        json!({"template": "Node 1 executed"}),
        DataProcessorMapping::default(),
    );
    let node2 = Node::new(
        "node2",
        NodeType::Data(DataNode::Prompt),
        json!({"template": "Node 2 executed"}),
        DataProcessorMapping::default(),
    );

    // 构建 Graph
    let mut graph = Graph::new();
    graph.add_node_data(node1);
    graph.add_node_data(node2);
    graph.add_edge("node1", "node2").unwrap();

    // 输入数据
    let input = json!("Start");

    // 启动工作流
    Workflow::start(graph, input).await.unwrap();
}
