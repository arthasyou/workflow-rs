use std::{io::Write, time::Duration};

use futures_util::StreamExt;
use model_client::{
    clients::openai::OpenAITextClient,
    sdk::openai::{ChatMessage, Role},
    traits::{ModelClient, StreamModelClient},
};
use tokio::time::sleep;

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
    let mut stream = client.infer_stream(messages).await.unwrap();
    println!("\n== 流式响应 ==\n");

    while let Some(chunk) = stream.next().await {
        let data = chunk?;
        let s = std::str::from_utf8(&data).unwrap();

        for line in s.lines().filter(|l| !l.trim().is_empty()) {
            let line = line
                .trim_start()
                .strip_prefix("data: ")
                .unwrap_or(line)
                .trim();

            if line == "[DONE]" {
                // println!();
                break;
            }

            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let Some(content) = json
                        .pointer("/message/content")
                        .or_else(|| json.pointer("/choices/0/delta/content"))
                        .and_then(|v| v.as_str())
                    {
                        print!("{}", content);
                        std::io::stdout().flush().unwrap();
                    }

                    if json.get("done").and_then(|v| v.as_bool()) == Some(true)
                        || json
                            .get("choices")
                            .and_then(|v| v.get(0))
                            .and_then(|v| v.get("finish_reason"))
                            .is_some()
                    {
                        // println!();
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("Parse error: {}", err);
                }
            }
        }

        // 可选：稍作等待，避免拉取过快影响显示
        sleep(Duration::from_millis(20)).await;
    }
    // print_stream_chunks(stream).await?;

    Ok(())
}
