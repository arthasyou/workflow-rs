use serde::{Deserialize, Serialize};

pub type ImageMatrix = Vec<f32>;
pub type AudioFeatures = Vec<f32>;
pub type VideoFrames = Vec<Vec<u8>>;

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
pub enum SingleData {
    /// 文本数据
    Text(String),

    /// 文件数据，包括路径和类型
    FilePath(File),

    /// 图像特征向量
    ImageMatrix(ImageMatrix),

    /// 音频特征向量
    AudioFeatures(AudioFeatures),

    /// 视频帧序列
    VideoFrames(VideoFrames),
}

impl SingleData {
    pub fn new_text(text: &str) -> Self {
        SingleData::Text(text.to_string())
    }

    pub fn new_image_matrix(features: ImageMatrix) -> Self {
        SingleData::ImageMatrix(features)
    }

    pub fn new_audio_features(features: AudioFeatures) -> Self {
        SingleData::AudioFeatures(features)
    }

    pub fn new_video_frames(frames: VideoFrames) -> Self {
        SingleData::VideoFrames(frames)
    }

    pub fn new_file(path: &str, file_type: FileType) -> Self {
        SingleData::FilePath(File {
            path: path.to_string(),
            file_type,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataPayload {
    /// 单一数据类型
    Single(SingleData),

    /// 复合数据类型
    Collection(DataCollection),
}

impl Default for DataPayload {
    fn default() -> Self {
        DataPayload::Single(SingleData::Text("".into()))
    }
}

impl DataPayload {
    /// 创建单一数据类型
    pub fn new_single(data: SingleData) -> Self {
        DataPayload::Single(data)
    }

    /// 创建复合数据类型
    pub fn new_collection() -> Self {
        DataPayload::Collection(DataCollection::new())
    }
    /// 合并两个 `DataPayload`
    pub fn merge(self, other: DataPayload) -> DataPayload {
        match (self, other) {
            // 如果 `self` 是 `Collection`，则将 `other` 合并进去
            (DataPayload::Collection(mut col), other) => {
                col.merge(other);
                DataPayload::Collection(col)
            }

            // 如果 `other` 是 `Collection`，则将 `self` 合并进去
            (payload, DataPayload::Collection(mut col)) => {
                col.merge(payload);
                DataPayload::Collection(col)
            }

            // 如果两者都是 `Single`，则创建新的 `Collection`
            (DataPayload::Single(data1), DataPayload::Single(data2)) => {
                let mut col = DataCollection::new();
                col.merge(DataPayload::Single(data1));
                col.merge(DataPayload::Single(data2));
                DataPayload::Collection(col)
            }
        }
    }
}

impl DataPayload {
    /// 判断 `DataPayload` 是否为 `Text` 类型
    pub fn is_text(&self) -> bool {
        matches!(self, DataPayload::Single(SingleData::Text(_)))
    }

    /// 获取 `Text` 数据，如果不是 `Text` 类型则返回 `None`
    pub fn as_text(&self) -> Option<&str> {
        if let DataPayload::Single(SingleData::Text(text)) = self {
            Some(text)
        } else {
            None
        }
    }

    /// 判断 `DataPayload` 是否为 `FilePath` 类型
    pub fn is_file(&self) -> bool {
        matches!(self, DataPayload::Single(SingleData::FilePath(_)))
    }

    /// 获取 `FilePath` 数据，如果不是 `FilePath` 类型则返回 `None`
    pub fn as_file(&self) -> Option<&File> {
        if let DataPayload::Single(SingleData::FilePath(file)) = self {
            Some(file)
        } else {
            None
        }
    }

    /// 判断 `DataPayload` 是否为 `Collection` 类型
    pub fn is_collection(&self) -> bool {
        matches!(self, DataPayload::Collection(_))
    }

    /// 获取 `DataCollection`，如果不是 `Collection` 类型则返回 `None`
    pub fn as_collection(&self) -> Option<&DataCollection> {
        if let DataPayload::Collection(collection) = self {
            Some(collection)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCollection {
    pub texts: Vec<String>,
    pub files: Vec<File>,
    pub image_matrices: Vec<ImageMatrix>,
    pub audio_features: Vec<AudioFeatures>,
    pub video_frames: Vec<VideoFrames>,
}

impl DataCollection {
    /// 创建空集合
    pub fn new() -> Self {
        Self {
            texts: Vec::new(),
            files: Vec::new(),
            image_matrices: Vec::new(),
            audio_features: Vec::new(),
            video_frames: Vec::new(),
        }
    }

    /// 合并 `DataPayload`
    pub fn merge(&mut self, payload: DataPayload) {
        match payload {
            DataPayload::Single(data) => self.merge_single(data),
            DataPayload::Collection(collection) => self.merge_collection(collection),
        }
    }

    /// 合并单一数据类型 `SingleData`
    pub fn merge_single(&mut self, data: SingleData) {
        match data {
            SingleData::Text(text) => self.texts.push(text),
            SingleData::FilePath(file) => self.files.push(file),
            SingleData::ImageMatrix(matrix) => self.image_matrices.push(matrix),
            SingleData::AudioFeatures(features) => self.audio_features.push(features),
            SingleData::VideoFrames(frames) => self.video_frames.push(frames),
        }
    }

    /// 合并 `DataCollection`
    fn merge_collection(&mut self, other: DataCollection) {
        self.texts.extend(other.texts);
        self.files.extend(other.files);
        self.image_matrices.extend(other.image_matrices);
        self.audio_features.extend(other.audio_features);
        self.video_frames.extend(other.video_frames);
    }
}
