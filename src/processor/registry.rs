use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;

use super::implementations::{logging::LoggingProcessor, validation::ValidationProcessor};
use crate::processor::{InputProc, OutputProc};

/// Processor Registry
#[derive(Default)]
pub struct ProcessorRegistry {
    input_processors: Mutex<HashMap<String, InputProc>>,
    output_processors: Mutex<HashMap<String, OutputProc>>,
}

impl ProcessorRegistry {
    pub fn register_input(&self, node_kind: &str, processor: InputProc) {
        let mut processors = self.input_processors.lock().unwrap();
        processors.insert(node_kind.to_string(), processor);
    }

    pub fn register_output(&self, node_kind: &str, processor: OutputProc) {
        let mut processors = self.output_processors.lock().unwrap();
        processors.insert(node_kind.to_string(), processor);
    }

    pub fn get_input(&self, node_kind: &str) -> InputProc {
        let processors = self.input_processors.lock().unwrap();
        processors.get(node_kind).cloned().unwrap_or(None)
    }

    pub fn get_output(&self, node_kind: &str) -> OutputProc {
        let processors = self.output_processors.lock().unwrap();
        processors.get(node_kind).cloned().unwrap_or(None)
    }
}

pub static PROCESSOR_REGISTRY: Lazy<ProcessorRegistry> = Lazy::new(ProcessorRegistry::default);

pub fn register_default_processors(registry: &ProcessorRegistry) {
    registry.register_input("Prompt", Some(Arc::new(LoggingProcessor)));
    registry.register_output("Prompt", Some(Arc::new(ValidationProcessor)));

    registry.register_input("Model", Some(Arc::new(LoggingProcessor)));
    registry.register_output("Model", Some(Arc::new(ValidationProcessor)));

    registry.register_input("Aggregator", Some(Arc::new(LoggingProcessor)));
    registry.register_output("Aggregator", Some(Arc::new(ValidationProcessor)));

    registry.register_input("Transformer", Some(Arc::new(LoggingProcessor)));
    registry.register_output("Transformer", Some(Arc::new(ValidationProcessor)));
}
