use async_trait::async_trait;
use service_utils_rs::utils::ByteStream;

use crate::error::Result; // Assumes ByteStream is defined elsewhere

/// Trait for normal model inference
#[async_trait]
pub trait ModelClient {
    type Input;
    type Output;

    async fn infer(&self, input: Self::Input) -> Result<Self::Output>;
}

/// Trait for stream-based model inference
#[async_trait]
pub trait StreamModelClient {
    type Input;

    async fn infer_stream(&self, input: Self::Input) -> Result<ByteStream>;
}
