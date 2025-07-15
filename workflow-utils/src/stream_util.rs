use std::io::Write;

use bytes::Bytes;
use futures_util::StreamExt;
use tokio::sync::mpsc::UnboundedSender;
use toolcraft::request::ByteStream;
use workflow_error::{Error, Result};

pub type StreamSender = UnboundedSender<(String, Bytes)>;

pub async fn print_stream_chunks(mut stream: ByteStream) -> Result<()> {
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
                        break;
                    }
                }
                Err(e) => eprintln!("Error parsing JSON: {}", e),
            }
        }
    }

    Ok(())
}

pub async fn forward_and_collect_stream(
    mut stream: ByteStream,
    tx: StreamSender,
    node: &str,
) -> Result<Bytes> {
    let mut collected_chunks = Vec::new();

    while let Some(chunk) = stream.next().await {
        match &chunk {
            Ok(bytes) => {
                tx.send((node.to_owned(), bytes.clone())).ok(); // forward
                collected_chunks.push(bytes.clone()); // collect
            }
            Err(e) => {
                return Err(Error::StreamChunkError(format!(
                    "stream chunk error: {}",
                    e
                )));
            }
        }
    }

    Ok(Bytes::from_iter(collected_chunks.into_iter().flatten()))
}
