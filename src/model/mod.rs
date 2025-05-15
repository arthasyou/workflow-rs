pub mod context;
pub mod graph_data;
pub mod input;
pub mod node;
pub mod output;

pub use context::Context;
pub use input::NodeInput;
pub use output::NodeOutput;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataPayload {
    Text(String),

    // 原始数据类型 (未经处理)
    RawImage(Vec<u8>),
    RawAudio(Vec<u8>),
    RawVideo(Vec<u8>),

    // 转化后的数据类型 (特征向量、矩阵等)
    ImageMatrix(Vec<f32>),     // 图片特征向量，如 CLIP 向量
    AudioFeatures(Vec<f32>),   // 音频特征，如 MFCC
    VideoFrames(Vec<Vec<u8>>), // 视频帧序列

    Control(String), // 控制信号，例如 "branch1:path1"
}
