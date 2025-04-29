use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{graph::Graph, storage::GraphStorage};

/// 仅用于测试或开发阶段的内存版存储
pub struct MockStorage {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl GraphStorage for MockStorage {
    fn save_graph(&self, id: &str, graph: &Graph) -> Result<(), Box<dyn std::error::Error>> {
        let json = graph.to_json();
        let mut store = self.store.lock().unwrap();
        store.insert(id.to_string(), json);
        Ok(())
    }

    fn load_graph(&self, id: &str) -> Result<Graph, Box<dyn std::error::Error>> {
        let store = self.store.lock().unwrap();
        let json = store.get(id).ok_or("Graph not found")?;
        let graph = Graph::from_json(json)?;
        Ok(graph)
    }
}
