use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use workflow_error::{Error, Result};

use crate::{
    model::{Context, DataPayload, output::OutputData},
    node::NodeBase,
};

/// 节点执行器 trait
#[async_trait]
pub trait Executable: Send + Sync + Debug {
    /// 获取 NodeBase 引用
    fn get_base(&self) -> &NodeBase;

    /// 输入处理逻辑 - 仅限当前节点，不涉及其他节点
    async fn process_input(&self, input: Option<DataPayload>) -> Option<DataPayload> {
        self.get_base().process_input(input).await
    }

    /// 核心执行逻辑 - 可访问其他节点实例
    async fn core_execute(
        &self,
        input: Option<DataPayload>,
        context: Arc<Context>,
    ) -> Result<OutputData>;

    /// 输出处理逻辑 - 仅限当前节点，不涉及其他节点
    async fn process_output(&self, output: OutputData) -> Option<OutputData> {
        self.get_base().process_output(output).await
    }

    /// 统一执行流程 - 内部传递 Context，仅 `core_execute` 使用 Context
    async fn execute(
        &self,
        input: Option<DataPayload>,
        context: Arc<Context>,
    ) -> Result<OutputData> {
        let processed_input = self.process_input(input).await;
        let output = self.core_execute(processed_input, context).await?;
        self.process_output(output)
            .await
            .ok_or_else(|| Error::ExecutionError("process_output returned None".into()))
    }

    /// 克隆自身并返回 Box<dyn Executable>
    fn clone_box(&self) -> Box<dyn Executable>;
}

/// 为 Box<dyn Executable> 实现 Clone
impl Clone for Box<dyn Executable> {
    fn clone(&self) -> Box<dyn Executable> {
        self.clone_box()
    }
}
