pub mod mock;

use crate::graph::Graph;

pub trait GraphStorage {
    fn save_graph(&self, id: &str, graph: &Graph) -> Result<(), Box<dyn std::error::Error>>;
    fn load_graph(&self, id: &str) -> Result<Graph, Box<dyn std::error::Error>>;
}
