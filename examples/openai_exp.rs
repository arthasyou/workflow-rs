use mcp_client::client;
use model_gateway_rs::{
    clients::llm::LlmClient,
    model::llm::{ChatMessage, LlmInput, Role},
    sdk::openai::OpenAIClient,
    traits::{ModelClient, StreamModelClient},
};
use workflow_utils::stream_util::print_stream_chunks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 OpenAIClient（使用 Ollama 本地服务）
    let api_key = ""; // Ollama 不需要 API key
    let base_url = "http://localhost:11434/v1";
    let model = "llama3.2";

    let sdk = OpenAIClient::new(api_key, base_url, model).unwrap();
    let client = LlmClient::new(sdk);

    // 准备 ChatMessage
    let messages = vec![ChatMessage {
        role: Role::User,
        content: "请介绍一下你自己".to_string(),
    }];

    let input = LlmInput {
        messages: messages.clone(),
        max_tokens: Some(100),
    };

    // 普通调用示例
    let resp = client.infer(input.clone()).await.unwrap();
    println!("\n== 普通响应 ==\n{}", resp.get_content());

    // 流式调用示例
    let stream = client.infer_stream(input).await.unwrap();
    // println!("\n== 流式响应 ==\n");

    print_stream_chunks(stream).await.unwrap();

    Ok(())
}
