use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 用于将 HashMap 映射为 JSON 列表形式的中间结构
#[derive(Serialize, Deserialize)]
pub struct KeyValue<K, V> {
    pub key: K,
    pub value: V,
}

/// 自定义序列化函数：将 HashMap<K, V> 序列化为 Vec<KeyValue<K, V>>
pub fn serialize_as_list<S, K, V>(map: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    K: std::fmt::Display + Eq + std::hash::Hash,
    V: Serialize,
{
    let list: Vec<_> = map
        .iter()
        .map(|(k, v)| KeyValue {
            key: k.to_string(),
            value: v,
        })
        .collect();
    list.serialize(serializer)
}

/// 自定义反序列化函数：将 Vec<KeyValue<K, V>> 反序列化为 HashMap<K, V>
pub fn deserialize_from_list<'de, D, K, V>(deserializer: D) -> Result<HashMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: std::str::FromStr + Eq + std::hash::Hash,
    <K as std::str::FromStr>::Err: std::fmt::Display,
    V: Deserialize<'de>,
{
    let kv_list: Vec<KeyValue<String, V>> = Vec::deserialize(deserializer)?;
    let mut map = HashMap::new();
    for KeyValue { key, value } in kv_list {
        let parsed_key = key.parse::<K>().map_err(serde::de::Error::custom)?;
        map.insert(parsed_key, value);
    }
    Ok(map)
}
