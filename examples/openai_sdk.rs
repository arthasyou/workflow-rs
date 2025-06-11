use std::{io::Write, time::Duration};

use futures_util::StreamExt;
use model_client::sdk::openai::{ChatMessage, ChatRequest, OpenAIClient, Role};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "your_openai_api_key";
    let base_url = "http://localhost:11434"; // Change to your OpenAI API base URL if needed

    let client = OpenAIClient::new(api_key, base_url)?;

    let req = ChatRequest {
        model: "llama3.2".to_string(),
        messages: vec![ChatMessage {
            role: Role::User,
            content: "Hello, who are you?".to_string(),
        }],
        stream: None,
        temperature: Some(0.7),
    };

    let resp = client.chat_once(req).await?;
    println!("Response: {}", resp);

    // 流式请求
    let stream_req = ChatRequest {
        model: "llama3.2".to_string(),
        messages: vec![ChatMessage {
            role: Role::User,
            content: "Can you stream your response?".to_string(),
        }],
        stream: Some(true),
        temperature: Some(0.7),
    };

    let mut stream = client.chat_stream(stream_req).await?;
    println!("Streaming response:");

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

    Ok(())
}
