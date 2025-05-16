use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::edge::Edge;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    Image,
    Audio,
    Video,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub file_type: FileType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataPayload {
    /// 文本数据
    Text(String),

    /// 文件数据，包括路径和类型
    FilePath(File),

    /// 转化后的数据类型 (特征向量、矩阵等)
    ImageMatrix(Vec<f32>),
    AudioFeatures(Vec<f32>),
    VideoFrames(Vec<Vec<u8>>),
}

impl DataPayload {
    /// 创建文本数据
    pub fn new_text(text: &str) -> Self {
        DataPayload::Text(text.to_string())
    }

    /// 创建图片特征向量
    pub fn new_image_matrix(features: Vec<f32>) -> Self {
        DataPayload::ImageMatrix(features)
    }

    /// 创建音频特征向量
    pub fn new_audio_features(features: Vec<f32>) -> Self {
        DataPayload::AudioFeatures(features)
    }

    /// 创建视频帧序列
    pub fn new_video_frames(frames: Vec<Vec<u8>>) -> Self {
        DataPayload::VideoFrames(frames)
    }

    /// 创建文件路径数据
    pub fn with_file_path(path: &str, file_type: FileType) -> Self {
        DataPayload::FilePath(File {
            path: path.to_string(),
            file_type,
        })
    }
}
