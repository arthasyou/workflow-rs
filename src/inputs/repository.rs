use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

use crate::model::DataPayload;

/// 输入数据仓库，用于存储和管理节点输入数据
#[derive(Debug, Default)]
pub struct InputRepository {
    storage: Mutex<HashMap<String, DataPayload>>,
}

impl InputRepository {
    /// 添加输入数据，返回数据 ID
    pub fn add(&self, data: DataPayload) -> String {
        let data_id = Uuid::new_v4().to_string();
        self.storage.lock().unwrap().insert(data_id.clone(), data);
        data_id
    }

    /// 批量初始化数据，覆盖现有数据
    pub fn init(&self, data_map: HashMap<String, DataPayload>) {
        let mut storage = self.storage.lock().unwrap();
        *storage = data_map;
    }

    /// 获取输入数据
    pub fn get(&self, data_id: &str) -> Option<DataPayload> {
        self.storage.lock().unwrap().get(data_id).cloned()
    }

    /// 更新输入数据
    pub fn update(&self, data_id: &str, data: DataPayload) -> bool {
        let mut storage = self.storage.lock().unwrap();
        if storage.contains_key(data_id) {
            storage.insert(data_id.to_string(), data);
            true
        } else {
            false
        }
    }

    /// 删除输入数据
    pub fn remove(&self, data_id: &str) -> bool {
        self.storage.lock().unwrap().remove(data_id).is_some()
    }
}
