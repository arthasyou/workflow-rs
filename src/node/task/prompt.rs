// src/task/prompt.rs

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{model::Context, node::Executable, processor::InputProcessor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptNode {
    pub id: String,
    pub prompt: String,
}

// #[async_trait]
// impl Executable for PromptNode {
//     async fn execute(&self, context: &mut Context) -> Result<(), String> {
//         let input = context.get_input(self.id.clone()).unwrap_or_default();
//         let processed_input = InputProc::process(input)?;

//         let result = format!("PromptNode - Processing: {}", processed_input);
//         context.set_output(self.id.clone(), result.clone());

//         Ok(())
//     }
// }
