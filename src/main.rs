use async_trait::async_trait;
use enchanter_core::prelude::*;
use enchanter_core::tools::tool::Tool;

struct MyTool;

impl Default for MyTool {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for MyTool {
    fn name(&self) -> &str {
        "get_capital"
    }

    fn description(&self) -> &str {
        "Look up the capital of a country."
    }

    fn schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "country": {
                    "type": "string",
                    "description": "The country to look up."
                }
            },
            "required": ["country"]
        })
    }

    async fn execute(
        &self,
        arguments: serde_json::Value,
        _context: &ToolContext,
    ) -> anyhow::Result<String> {
        let country = arguments
            .get("country")
            .and_then(|value| value.as_str())
            .unwrap_or("France");
        Ok(format!("The capital of {country} is Paris."))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _model = std::env::var("ENCHANTER_MODEL").unwrap_or_else(|_| "gemma4:12b".into());

    let mut agent = Agent::from_env()?;
    agent.register_tool(MyTool::default());

    let prompt = "What is the capital of France? Answer in one sentence.";

    println!("User prompt: {prompt}");

    match agent.chat(prompt).await {
        Ok(response) => {
            println!("Response: {}", response.content());
            if let Some(usage) = response.usage {
                println!();
                println!(
                    "Tokens — prompt: {}, completion: {}, total: {}",
                    usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
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
