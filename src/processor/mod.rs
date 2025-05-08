pub mod processors;
pub mod registry;
pub mod traits;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;
use serde_json::Value;

use crate::error::Result;

pub type Processor<T> = Option<Arc<dyn ProcessorTrait<T> + Send + Sync>>;

/// 通用 Processor Trait
pub trait ProcessorTrait<T> {
    fn process(&self, node_id: &str, data: &T, context: Option<&Value>) -> Result<T>;
}

/// 输入和输出 Processor 类型
pub type InputProc = Processor<Value>;
pub type OutputProc = Processor<Value>;

/// Processor Registry
#[derive(Default)]
pub struct ProcessorRegistry {
    input_processors: Mutex<HashMap<String, InputProc>>,
    output_processors: Mutex<HashMap<String, OutputProc>>,
}

impl ProcessorRegistry {
    /// 注册 InputProcessor
    pub fn register_input(&self, node_kind: &str, processor: InputProc) {
        let mut processors = self.input_processors.lock().unwrap();
        processors.insert(node_kind.to_string(), processor);
    }

    /// 注册 OutputProcessor
    pub fn register_output(&self, node_kind: &str, processor: OutputProc) {
        let mut processors = self.output_processors.lock().unwrap();
        processors.insert(node_kind.to_string(), processor);
    }

    /// 获取 InputProcessor
    pub fn get_input(&self, node_kind: &str) -> InputProc {
        let processors = self.input_processors.lock().unwrap();
        processors.get(node_kind).cloned().unwrap_or(None)
    }

    /// 获取 OutputProcessor
    pub fn get_output(&self, node_kind: &str) -> OutputProc {
        let processors = self.output_processors.lock().unwrap();
        processors.get(node_kind).cloned().unwrap_or(None)
    }
}

pub static PROCESSOR_REGISTRY: Lazy<ProcessorRegistry> = Lazy::new(ProcessorRegistry::default);
