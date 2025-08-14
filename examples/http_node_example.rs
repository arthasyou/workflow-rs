
use serde_json::json;
use workflow_rs::{
    Workflow,
    graph::Graph,
    model::node::{DataNode, DataProcessorMapping, Node, NodeType},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建节点
    let nodes = vec![
        // 开始节点 - 提供输入数据
        Node::new(
            "start",
            NodeType::Data(DataNode::Input),
            json!({
                "input": {
                    "type": "Single",
                    "value": {
                        "type": "Json",
                        "value": {
                            "user_id": "12345",
                            "action": "get_profile"
                        }
                    }
                }
            }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
        // HTTP 请求节点
        Node::new(
            "http_node",
            NodeType::Data(DataNode::Http),
            json!({
                "url": "http://192.168.1.233:19876/auth/login",
                "input_data": {
                    "username": "test",
                    "password": "123"
                },
                "method": "POST",
                "headers": {
                    "Accept": "application/json"
                },
                "timeout_seconds": 10
            }),
            DataProcessorMapping::default(),
            None,
            None,
        ),
        // 结束节点
        Node::new_end(),
    ];

    // 创建边
    let edges = vec![("start", "http_node"), ("http_node", "end")];

    let mut graph = Graph::new();
    for node in nodes {
        graph.add_node(node).unwrap();
    }

    for (start, end) in edges {
        graph.add_edge(start, end, None, None).unwrap();
    }

    // Execute the workflow
    let r = Workflow::start(graph).await;
    
    match r {
        Ok(result) => println!("Graph execution succeeded! Result: {:?}", result),
        Err(e) => println!("Graph execution error: {:?}", e),
    }

    Ok(())
}
