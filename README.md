# Enchanter CORE

> A minimal, modular, and extensible harness for building OpenAI-compatible agents in Rust.

---

## What is Enchanter CORE?

Enchanter CORE is a Rust library for building LLM-powered agents. It provides a clean, typed foundation for talking to any OpenAI-compatible model endpoint — whether that is OpenAI itself, Ollama, LM Studio, vLLM, or any other compatible server.

The design philosophy is simple:

- **Minimal** — no magic, no hidden state, no framework lock-in.
- **Modular** — use only what you need. Each layer is independently replaceable.
- **Extensible** — the trait system is designed to grow. Add providers, tools, memory backends, and MCP support without touching the core.

---

## Current State

The library is in early but solid foundational shape. The following modules are complete and tested:

### `protocol/`
The OpenAI Chat Completions protocol, fully typed and serialization-tested.

| File | Contents |
|---|---|
| `message.rs` | `Role`, `ChatMessage` |
| `chat.rs` | `ChatRequest`, `ChatRequestBuilder` |
| `response.rs` | `ChatResponse`, `ChatStreamChunk`, `Usage` |
| `tool.rs` | `Tool`, `ToolCall`, `ToolChoice` |

### `provider/`
The transport layer. Providers take a `ChatRequest` and return a `ChatResponse`.

| File | Contents |
|---|---|
| `mod.rs` | `LlmProvider`, `Streamable`, `ChatStream` traits |
| `openai.rs` | HTTP transport for any OpenAI-compatible endpoint |
| `mock.rs` | In-memory mock provider for testing |

### Tests
All tests live in dedicated `tests/` subdirectories, completely separate from implementation files:

```
src/protocol/tests/
    mod.rs
    message.rs
    chat.rs
    response.rs
    tool.rs
```

---

## Project Structure

```
enchanter-core/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── main.rs                  ← local model integration test
    ├── protocol/
    │   ├── mod.rs
    │   ├── message.rs
    │   ├── chat.rs
    │   ├── response.rs
    │   ├── tool.rs
    │   └── tests/
    │       ├── mod.rs
    │       ├── message.rs
    │       ├── chat.rs
    │       ├── response.rs
    │       └── tool.rs
    └── provider/
        ├── mod.rs
        ├── openai.rs
        ├── mock.rs
        └── tests/
            ├── mod.rs
            └── mock.rs
```

---

## Getting Started

### Prerequisites

- Rust 1.80+ (edition 2024)
- A running local model server, e.g. [Ollama](https://ollama.com)

### Clone and build

```bash
git clone https://github.com/your-org/enchanter-core
cd enchanter-core
cargo build
```

### Run the tests

```bash
cargo test
```

### Run against a local model

The `main.rs` binary connects to a local OpenAI-compatible endpoint and sends a single test message.

```bash
cargo run
```

By default it connects to Ollama at `http://localhost:11434/v1` using `llama3.2`. Override with environment variables:

```bash
ENCHANTER_BASE_URL=http://localhost:1234/v1 \
ENCHANTER_MODEL=mistral \
cargo run
```

---

## Using Enchanter CORE

### Add it as a dependency

```toml
[dependencies]
enchanter-core = { path = "../enchanter-core" }
```

### Build a request

```rust
use enchanter_core::prelude::*;

let request = ChatRequest::builder("llama3.2")
    .system("You are a helpful assistant.")
    .user("What is the capital of France?")
    .temperature(0.7)
    .max_tokens(256)
    .build();
```

### Send it to a provider

```rust
use enchanter_core::prelude::*;
use enchanter_core::provider::openai::OpenAiProvider;

let provider = OpenAiProvider::new("http://localhost:11434/v1", "llama3.2");

let response = provider.complete(request).await?;
println!("{}", response.content());
```

### Write your own provider

Implement the `LlmProvider` trait for any backend:

```rust
use async_trait::async_trait;
use enchanter_core::prelude::*;
use enchanter_core::provider::LlmProvider;

pub struct MyProvider;

#[async_trait]
impl LlmProvider for MyProvider {
    async fn complete(&self, request: ChatRequest) -> anyhow::Result<ChatResponse> {
        // your transport logic here
        todo!()
    }
}
```

### Build a simple agent loop

A basic agent loop is just a `while` loop over `complete()` calls:

```rust
use enchanter_core::prelude::*;
use enchanter_core::provider::openai::OpenAiProvider;

async fn run_agent(provider: &impl LlmProvider) -> anyhow::Result<()> {
    let mut messages = vec![
        ChatMessage::system("You are a helpful assistant."),
    ];

    loop {
        // get user input
        let input = "Paris"; // Simulated input
        if input.trim() == "exit" { break; }

        messages.push(ChatMessage::user(&input));

        let request = ChatRequest::builder("llama3.2")
            .messages(messages.clone())
            .build();

        let response = provider.complete(request).await?;

        println!("Assistant: {}", response.content());
        messages.push(response.message);

        if response.is_complete() {
            // natural stop — continue the loop
        }
    }

    Ok(())
}
```

### Add a tool

```rust
use enchanter_core::prelude::*;
use serde_json::json;

let tool = Tool::function(
    "get_weather",
    "Get the current weather for a location.",
    json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "City and country, e.g. 'Paris, France'"
            }
        },
        "required": ["location"]
    }),
);

let request = ChatRequest::builder("llama3.2")
    .system("You are a helpful assistant.")
    .user("What is the weather in Tokyo?")
    .tool(tool)
    .tool_choice(ToolChoice::Auto)
    .build();
```

---

## Roadmap

### Near term

| Module | Description |
|---|---|
| `tool/` | `Tool` trait, `ToolRegistry`, automatic dispatch in the agent loop |
| `agent/` | A proper `Agent` struct that owns the loop, history, and tool dispatch |
| `memory/` | Conversation memory backends — in-memory, SQLite, and rolling window |

### Medium term

| Module | Description |
|---|---|
| `provider/ollama.rs` | Native Ollama provider (non-OpenAI-compat path) |
| `provider/anthropic.rs` | Anthropic Claude provider |
| `streaming` | Full streaming support in the agent loop via `Streamable` |
| `mcp/` | Model Context Protocol — spawn and communicate with MCP servers |

---

## Design Principles

1. **Protocol and transport are separate.** `protocol/` owns the types. `provider/` owns the HTTP.
2. **Traits over structs.** `LlmProvider` and `Streamable` traits mean swapable backends.
3. **Tests are first-class.** Dedicated `tests/` directory for every module.
4. **The prelude is your friend.** Fast access to 90% of use cases.
5. **No magic.** Traceable, explicit, and framework-free.

---

## License

MIT