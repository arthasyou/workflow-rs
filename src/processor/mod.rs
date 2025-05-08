pub mod implementations;
pub mod registry;
pub mod traits;

pub use registry::{PROCESSOR_REGISTRY, ProcessorRegistry};
pub use traits::{InputProc, OutputProc, Processor, ProcessorTrait};
