use async_trait::async_trait;

use crate::error::Result;

#[async_trait]
pub trait ModelClient {
    type Input;
    type Output;

    async fn infer(&self, input: Self::Input) -> Result<Self::Output>;
}
