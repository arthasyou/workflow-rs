pub mod compiler;
pub mod edge;
pub mod graph;
pub mod node;
pub mod runner;
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
        });
        g.add_node(Node {
            id: "model".into(),
            kind: NodeKind::Model,
        });

        g.add_edge("prompt", "model");

        println!("Graph nodes: {:?}", g.nodes);
        println!("Graph edges: {:?}", g.edges);
    }
}
