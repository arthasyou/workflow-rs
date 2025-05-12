use std::{collections::HashMap, sync::Arc};

use super::node::Node;
use crate::node::{Executable, builder::build_node};

/// Context：运行时节点实例管理器
#[derive(Debug, Clone)]
pub struct Context {
    pub nodes: HashMap<String, Arc<dyn Executable>>,
    pub metadata: HashMap<String, String>,
}

impl Context {
    /// 根据 Graph 的 `node_data` 生成节点实例并存储到 Context
    pub fn new(node_data: &HashMap<String, Node>) -> Self {
        let mut nodes = HashMap::new();

        for (id, node) in node_data {
            // 根据 Node 数据生成节点实例
            if let Ok(instance) = build_node(node) {
                nodes.insert(id.clone(), instance);
            }
        }

        Self {
            nodes,
            metadata: HashMap::new(),
        }
    }

    /// 获取节点实例
    pub fn get_node(&self, id: &str) -> Option<&Arc<dyn Executable>> {
        self.nodes.get(id)
    }

    /// 设置元数据
    pub fn set_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    /// 获取元数据
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}
