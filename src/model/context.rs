use std::{collections::HashMap, sync::Arc};

use crate::{
    graph::Graph,
    node::{Executable, builder::build_node},
};

/// Context：运行时节点实例管理器
#[derive(Debug, Clone)]
pub struct Context {
    pub nodes: HashMap<String, Arc<dyn Executable>>,
    pub metadata: HashMap<String, String>,
}

impl Context {
    /// 根据 Graph 生成节点实例并存储到 Context
    pub fn from_graph(graph: &Graph) -> Arc<Self> {
        let mut nodes = HashMap::new();

        for (id, node) in &graph.nodes {
            if let Ok(instance) = build_node(node) {
                nodes.insert(id.clone(), instance);
            }
        }

        Arc::new(Self {
            nodes,
            metadata: HashMap::new(),
        })
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

/// 接收 token 的推送目标接口，由外部系统实现（如 SSE/WebSocket）
pub trait TokenSink: Send + Sync {
    fn push_token(&self, token: &str);
}

/// 运行期上下文，包含可选的 token sink 用于推流
#[derive(Default, Clone)]
pub struct RunContext {
    pub token_sink: Option<Arc<dyn TokenSink>>,
}
