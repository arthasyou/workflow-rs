use toolcraft_request::ByteStream;
use workflow_error::{Error, Result};

use crate::{FlowData, FlowOutputType};

pub struct ControlFlow {
    pub next_node: String,
    pub data: FlowData,
}

impl ControlFlow {
    pub fn new(next_node: &str, data: FlowData) -> Self {
        Self {
            next_node: next_node.to_string(),
            data,
        }
    }
}

pub enum FlowOutputValue {
    Data(FlowData),
    Stream(ByteStream),
    Control(ControlFlow),
    Parallel(Vec<ControlFlow>),
}

pub struct FlowOutput {
    pub output_type: FlowOutputType,
    pub value: FlowOutputValue,
}

impl FlowOutput {
    pub fn new_data(data: FlowData) -> Self {
        Self {
            output_type: FlowOutputType::Data,
            value: FlowOutputValue::Data(data),
        }
    }

    pub fn new_control(next_node: &str, data: FlowData) -> Self {
        Self {
            output_type: FlowOutputType::Control,
            value: FlowOutputValue::Control(ControlFlow::new(next_node, data)),
        }
    }

    pub fn new_stream(stream: ByteStream) -> Self {
        Self {
            output_type: FlowOutputType::Stream,
            value: FlowOutputValue::Stream(stream),
        }
    }

    pub fn as_data(&self) -> Result<&FlowData> {
        match &self.value {
            FlowOutputValue::Data(data) => Ok(data),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_control(&self) -> Result<&ControlFlow> {
        match &self.value {
            FlowOutputValue::Control(ctrl) => Ok(ctrl),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_stream(&self) -> Result<&ByteStream> {
        match &self.value {
            FlowOutputValue::Stream(stream) => Ok(stream),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_data(self) -> Result<FlowData> {
        match self.value {
            FlowOutputValue::Data(data) => Ok(data),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_control(self) -> Result<ControlFlow> {
        match self.value {
            FlowOutputValue::Control(ctrl) => Ok(ctrl),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_stream(self) -> Result<ByteStream> {
        match self.value {
            FlowOutputValue::Stream(stream) => Ok(stream),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn get_type(&self) -> &FlowOutputType {
        &self.output_type
    }
}

impl From<FlowData> for FlowOutput {
    fn from(data: FlowData) -> Self {
        FlowOutput::new_data(data)
    }
}

impl From<ByteStream> for FlowOutput {
    fn from(stream: ByteStream) -> Self {
        FlowOutput::new_stream(stream)
    }
}

impl<T: Into<String>> From<(T, FlowData)> for FlowOutput {
    fn from((next_node, data): (T, FlowData)) -> Self {
        FlowOutput::new_control(&next_node.into(), data)
    }
}

impl From<Vec<ControlFlow>> for FlowOutput {
    fn from(parallel: Vec<ControlFlow>) -> Self {
        FlowOutput {
            output_type: FlowOutputType::Parallel,
            value: FlowOutputValue::Parallel(parallel),
        }
    }
}
