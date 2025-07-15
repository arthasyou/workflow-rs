use serde::{Deserialize, Serialize};
use serde_json::Value;
use workflow_error::{Error, Result};

use crate::FileType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum FlowData {
    Single(SingleData),
    Collection(Vec<SingleData>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SingleData {
    Text(String),
    Number(f64),
    File(FileValue),
    Json(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileValue {
    pub path: String,
    pub file_type: FileType,
}

/// Constructors for FlowData.
impl FlowData {
    pub fn new_collection() -> Self {
        Self::Collection(Vec::new())
    }

    pub fn try_from_json<T: Serialize>(value: T) -> Result<Self> {
        let json_value = serde_json::to_value(value)?;
        Ok(Self::Single(SingleData::Json(json_value)))
    }
}

impl From<String> for FlowData {
    fn from(text: String) -> Self {
        Self::Single(SingleData::Text(text))
    }
}

impl From<&str> for FlowData {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl From<f64> for FlowData {
    fn from(n: f64) -> Self {
        Self::Single(SingleData::Number(n))
    }
}

impl From<Value> for FlowData {
    fn from(val: Value) -> Self {
        Self::Single(SingleData::Json(val))
    }
}

/// Accessors (borrowed) for FlowData.
impl FlowData {
    pub fn as_text(&self) -> Result<&str> {
        match &self {
            Self::Single(SingleData::Text(s)) => Ok(s),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_number(&self) -> Result<f64> {
        match &self {
            Self::Single(SingleData::Number(n)) => Ok(*n),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_file(&self) -> Result<&FileValue> {
        match &self {
            Self::Single(SingleData::File(f)) => Ok(f),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_collection(&self) -> Result<&[SingleData]> {
        match &self {
            Self::Collection(vec) => Ok(vec),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_text_list(&self) -> Result<Vec<&str>> {
        match &self {
            Self::Single(SingleData::Text(s)) => Ok(vec![s.as_str()]),
            Self::Collection(vec) => {
                let mut result = Vec::with_capacity(vec.len());
                for item in vec {
                    match item {
                        SingleData::Text(s) => result.push(s.as_str()),
                        _ => return Err(Error::FlowTypeMismatch),
                    }
                }
                Ok(result)
            }
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_number_list(&self) -> Result<Vec<f64>> {
        match &self {
            Self::Single(SingleData::Number(n)) => Ok(vec![*n]),
            Self::Collection(vec) => {
                let mut result = Vec::with_capacity(vec.len());
                for item in vec {
                    match item {
                        SingleData::Number(n) => result.push(*n),
                        _ => return Err(Error::FlowTypeMismatch),
                    }
                }
                Ok(result)
            }
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn as_file_list(&self) -> Result<Vec<&FileValue>> {
        match &self {
            Self::Single(SingleData::File(f)) => Ok(vec![f]),
            Self::Collection(vec) => {
                let mut result = Vec::with_capacity(vec.len());
                for item in vec {
                    match item {
                        SingleData::File(f) => result.push(f),
                        _ => return Err(Error::FlowTypeMismatch),
                    }
                }
                Ok(result)
            }
            _ => Err(Error::FlowTypeMismatch),
        }
    }
}

/// Accessors (by value) for FlowData.
impl FlowData {
    pub fn into_text(self) -> Result<String> {
        match self {
            Self::Single(SingleData::Text(s)) => Ok(s),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_number(self) -> Result<f64> {
        match self {
            Self::Single(SingleData::Number(n)) => Ok(n),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_file(self) -> Result<FileValue> {
        match self {
            Self::Single(SingleData::File(f)) => Ok(f),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_collection(self) -> Result<Vec<SingleData>> {
        match self {
            Self::Collection(vec) => Ok(vec),
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_text_list(self) -> Result<Vec<String>> {
        match self {
            Self::Single(SingleData::Text(s)) => Ok(vec![s]),
            Self::Collection(vec) => {
                let mut result = Vec::with_capacity(vec.len());
                for item in vec {
                    match item {
                        SingleData::Text(s) => result.push(s),
                        _ => return Err(Error::FlowTypeMismatch),
                    }
                }
                Ok(result)
            }
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_number_list(self) -> Result<Vec<f64>> {
        match self {
            Self::Single(SingleData::Number(n)) => Ok(vec![n]),
            Self::Collection(vec) => {
                let mut result = Vec::with_capacity(vec.len());
                for item in vec {
                    match item {
                        SingleData::Number(n) => result.push(n),
                        _ => return Err(Error::FlowTypeMismatch),
                    }
                }
                Ok(result)
            }
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_file_list(self) -> Result<Vec<FileValue>> {
        match self {
            Self::Single(SingleData::File(f)) => Ok(vec![f]),
            Self::Collection(vec) => {
                let mut result = Vec::with_capacity(vec.len());
                for item in vec {
                    match item {
                        SingleData::File(f) => result.push(f),
                        _ => return Err(Error::FlowTypeMismatch),
                    }
                }
                Ok(result)
            }
            _ => Err(Error::FlowTypeMismatch),
        }
    }

    pub fn into_json(self) -> Result<serde_json::Value> {
        match self {
            Self::Single(SingleData::Json(json)) => Ok(json),
            _ => Err(Error::FlowTypeMismatch),
        }
    }
}

/// Utility methods for FlowData.
impl FlowData {
    pub fn merge(self, other: Self) -> Self {
        let merged = match self {
            Self::Collection(mut vec) => {
                match other {
                    Self::Single(data) => vec.push(data),
                    Self::Collection(mut other_vec) => vec.append(&mut other_vec),
                }
                Self::Collection(vec)
            }
            Self::Single(data1) => {
                let mut vec = vec![data1];
                match other {
                    Self::Single(data2) => vec.push(data2),
                    Self::Collection(mut other_vec) => vec.append(&mut other_vec),
                }
                Self::Collection(vec)
            }
        };
        merged
    }

    pub fn merge_mut(&mut self, other: Self) -> &mut Self {
        match self {
            Self::Collection(vec) => match other {
                Self::Single(data) => vec.push(data),
                Self::Collection(mut other_vec) => vec.append(&mut other_vec),
            },
            Self::Single(data1) => {
                let mut vec = vec![data1.clone()];
                match other {
                    Self::Single(data2) => vec.push(data2),
                    Self::Collection(mut other_vec) => vec.append(&mut other_vec),
                }
            }
        }
        self
    }
}
