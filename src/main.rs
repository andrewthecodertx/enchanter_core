use core::num;
use std::ptr::null;

use enchanter_core::prelude::*;
use enchanter_core::provider::openai::OpenAiProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_url = std::env::var("ENCHANTER_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:11434/v1".to_string());

    let model = std::env::var("ENCHANTER_MODEL").unwrap_or_else(|_| "gemma4:12b".to_string());

    println!("Connecting to: {base_url}");
    println!("Model: {model}");
    println!();

    let provider = OpenAiProvider::new(&base_url, None);

    let request = ChatRequest::builder(&model)
        .system("You are a helpful assistant. Be concise.")
        .user("What is the capital of France? Answer in one sentence.")
        .temperature(0.7)
        .max_tokens(128)
        .build();

    println!("Sending request...");

    match provider.complete(request).await {
        Ok(response) => {
            println!("Response: {}", response.content());
            if let Some(usage) = response.usage {
                println!();
                println!(
                    "Tokens — prompt: {}, completion: {}, total: {}",
                    usage.prompt_tokens, usage.completion_tokens, usage.total_tokens,
                );
            }
            if let Some(reason) = response.finish_reason {
                println!("Finish reason: {reason}");
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

