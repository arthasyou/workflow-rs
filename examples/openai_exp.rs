use model_client::{
    clients::openai::OpenAITextClient,
    sdk::openai::{ChatMessage, Role},
    traits::{ModelClient, StreamModelClient},
};
use workflow_utils::stream_util::print_stream_chunks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 OpenAIClient（使用 Ollama 本地服务）
    let api_key = ""; // Ollama 不需要 API key
    let base_url = "http://localhost:11434/v1";
    let model = "llama3.2";

    let client = OpenAITextClient::new(api_key, base_url, model).unwrap();

    // 准备 ChatMessage
    let messages = vec![ChatMessage {
        role: Role::User,
        content: "请介绍一下你自己".to_string(),
    }];

    // 普通调用示例
    let resp = client.infer(messages.clone()).await.unwrap();
    println!("\n== 普通响应 ==\n{}", resp.choices[0].message.content);

    // 流式调用示例
    let stream = client.infer_stream(messages).await.unwrap();
    println!("\n== 流式响应 ==\n");

    print_stream_chunks(stream).await?;

    Ok(())
}
