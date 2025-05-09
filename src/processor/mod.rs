pub mod implementations;
pub mod registry;
pub mod traits;

pub use registry::{PROCESSOR_REGISTRY, ProcessorRegistry, register_default_processors};
pub use traits::{InputProcessor, OutputProcessor, Processor};
