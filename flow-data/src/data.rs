use serde::{Deserialize, Serialize};
use workflow_error::{Error, Result};

use crate::{FileType, FlowDataType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowData {
    pub data_type: FlowDataType,
    pub value: FlowValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum FlowValue {
    Single(SingleData),
    Collection(Vec<SingleData>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SingleData {
    Text(String),
    Number(f64),
    File(FileValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileValue {
    pub path: String,
    pub file_type: FileType,
}

impl FlowData {
    pub fn new_text(text: impl Into<String>) -> Self {
        Self {
            data_type: FlowDataType::Text,
            value: FlowValue::Single(SingleData::Text(text.into())),
        }
    }

    pub fn new_number(n: f64) -> Self {
        Self {
            data_type: FlowDataType::Number,
            value: FlowValue::Single(SingleData::Number(n)),
        }
    }

    pub fn new_file(path: impl Into<String>, file_type: FileType) -> Self {
        Self {
            data_type: FlowDataType::File,
            value: FlowValue::Single(SingleData::File(FileValue {
                path: path.into(),
                file_type,
            })),
        }
    }

    pub fn new_collection() -> Self {
        Self {
            data_type: FlowDataType::Collection,
            value: FlowValue::Collection(Vec::new()),
        }
    }

    pub fn get_data_type(&self) -> &FlowDataType {
        &self.data_type
    }

    pub fn as_text(&self) -> Result<&str> {
        match &self.value {
            FlowValue::Single(SingleData::Text(s)) => Ok(s),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_number(&self) -> Result<f64> {
        match &self.value {
            FlowValue::Single(SingleData::Number(n)) => Ok(*n),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_collection(&self) -> Result<&[SingleData]> {
        match &self.value {
            FlowValue::Collection(vec) => Ok(vec),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_file(&self) -> Result<&FileValue> {
        match &self.value {
            FlowValue::Single(SingleData::File(f)) => Ok(f),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn merge(self, other: Self) -> Self {
        let merged = match self.value {
            FlowValue::Collection(mut vec) => {
                match other.value {
                    FlowValue::Single(data) => vec.push(data),
                    FlowValue::Collection(mut other_vec) => vec.append(&mut other_vec),
                }
                FlowValue::Collection(vec)
            }
            FlowValue::Single(data1) => {
                let mut vec = vec![data1];
                match other.value {
                    FlowValue::Single(data2) => vec.push(data2),
                    FlowValue::Collection(mut other_vec) => vec.append(&mut other_vec),
                }
                FlowValue::Collection(vec)
            }
        };
        FlowData {
            data_type: FlowDataType::Collection,
            value: merged,
        }
    }

    pub fn merge_mut(&mut self, other: Self) -> &mut Self {
        match &mut self.value {
            FlowValue::Collection(vec) => match other.value {
                FlowValue::Single(data) => vec.push(data),
                FlowValue::Collection(mut other_vec) => vec.append(&mut other_vec),
            },
            FlowValue::Single(data1) => {
                let mut vec = vec![data1.clone()];
                match other.value {
                    FlowValue::Single(data2) => vec.push(data2),
                    FlowValue::Collection(mut other_vec) => vec.append(&mut other_vec),
                }
                self.value = FlowValue::Collection(vec);
                self.data_type = FlowDataType::Collection;
            }
        }
        self
    }

    pub fn into_text(self) -> Result<String> {
        match self.value {
            FlowValue::Single(SingleData::Text(s)) => Ok(s),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_number(self) -> Result<f64> {
        match self.value {
            FlowValue::Single(SingleData::Number(n)) => Ok(n),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_file(self) -> Result<FileValue> {
        match self.value {
            FlowValue::Single(SingleData::File(f)) => Ok(f),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_collection(self) -> Result<Vec<SingleData>> {
        match self.value {
            FlowValue::Collection(vec) => Ok(vec),
            _ => Err(Error::FlowTypeMismatch),
        }
    }
}
