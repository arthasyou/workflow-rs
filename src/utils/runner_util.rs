use crate::model::{DataPayload, Node};

/// 合并两个 `DataPayload` 数据，用于累积多个输入数据。
pub fn merge_inputs(existing: DataPayload, new_data: DataPayload) -> DataPayload {
    let combined = existing.merge(new_data);
    combined
}

/// 检查节点是否是控制节点 (如 Branch, Repeat, Parallel)
pub fn is_control_node(node: &Node) -> bool {
    node.is_control_node()
}
