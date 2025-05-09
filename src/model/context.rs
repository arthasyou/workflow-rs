use std::{collections::HashMap, sync::Arc};

use crate::node::Executable;

/// 执行上下文，用于节点之间传递数据和状态
#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub nodes: &'a HashMap<String, Arc<dyn Executable>>,
    pub metadata: HashMap<String, String>,
}

impl<'a> Context<'a> {
    /// 创建新的 Context
    pub fn new(nodes: &'a HashMap<String, Arc<dyn Executable>>) -> Self {
        Self {
            nodes,
            metadata: HashMap::new(),
        }
    }

    /// 获取节点实例
    pub fn get_node(&self, id: &str) -> Option<&Arc<dyn Executable>> {
        self.nodes.get(id)
    }

    /// 设置上下文中的元数据
    pub fn set_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    /// 获取上下文中的元数据
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}
