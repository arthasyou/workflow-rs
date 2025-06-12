use async_trait::async_trait;

use crate::{
    traits::ModelClient,
    types::vision::{VisionPrompt, VisionResponse},
};

#[async_trait]
pub trait VisionGeneration: ModelClient<Input = VisionPrompt, Output = VisionResponse> {}
