pub mod edge;
pub mod error;
pub mod graph;
pub mod node;
pub mod processor;
pub mod runner;
pub mod storage;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::{
        graph::Graph,
        node::{Node, NodeKind},
    };

    #[test]
    fn simple_graph_test() {
        let mut g = Graph::new();

        g.add_node(Node {
            id: "prompt".into(),
            kind: NodeKind::Prompt,
            config: None,
            input_processor: None,
            output_processor: None,
        });
        g.add_node(Node {
            id: "model".into(),
            kind: NodeKind::Model,
            config: None,
            input_processor: None,
            output_processor: None,
        });

        let _ = g.add_edge("prompt", "model");

        println!("Graph nodes: {:?}", g.nodes);
        println!("Graph edges: {:?}", g.edges);
    }

    #[test]
    fn save_graph_as_json() {
        let mut g = Graph::new();
        g.add_node(Node {
            id: "prompt".into(),
            kind: NodeKind::Prompt,
            config: None,
            input_processor: None,
            output_processor: None,
        });
        g.add_node(Node {
            id: "model".into(),
            kind: NodeKind::Model,
            config: None,
            input_processor: None,
            output_processor: None,
        });
        let _ = g.add_edge("prompt", "model");

        let json = g.to_json();
        println!("{}", json);
    }

    #[test]
    fn save_and_load_graph() {
        let mut g = Graph::new();
        g.add_node(Node {
            id: "prompt".into(),
            kind: NodeKind::Prompt,
            config: None,
            input_processor: None,
            output_processor: None,
        });
        g.add_node(Node {
            id: "model".into(),
            kind: NodeKind::Model,
            config: None,
            input_processor: None,
            output_processor: None,
        });
        let _ = g.add_edge("prompt", "model");

        let json = g.to_json();
        println!("JSON:\n{}", json);

        let loaded = Graph::from_json(&json).expect("Failed to deserialize");

        assert_eq!(loaded.nodes.len(), 2);
        assert_eq!(loaded.edges.len(), 1);
        println!("Graph reloaded successfully: {:?}", loaded.nodes.keys());
    }
}
