# PrivacyGuard MCP

Private AI agent infrastructure for routing sensitive prompts to local LLMs.

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://rustup.rs)
[![MCP](https://img.shields.io/badge/MCP-Server-blue.svg)](https://modelcontextprotocol.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## What it does

- Classifies prompt text for sensitive keywords (PII/PHI/secrets)
- Routes sensitive prompts to local Ollama via MCP tool calls
- Emits JSON-RPC responses over stdio (MCP transport)
- Provides audit log retrieval from `audit.log`

## Status

This project is an early MVP. It currently exposes MCP tools over stdio only.

Available tools:
- `analyze_privacy`
- `route_to_local`
- `get_audit_logs`

## Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ollama (for local generation tool)
brew install ollama
ollama pull phi3:mini
ollama serve
```

## Build

```bash
cargo build --release
```

Binary path:

```bash
./target/release/privacyguard-mcp
```

## MCP client configuration example

```json
{
  "mcpServers": {
    "privacyguard": {
      "command": "/absolute/path/to/target/release/privacyguard-mcp"
    }
  }
}
```

## Local smoke test (stdio JSON-RPC)

```bash
./target/release/privacyguard-mcp <<'JSON'
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"analyze_privacy","arguments":{"text":"My password is secret123"}}}
JSON
```

## Development

```bash
cargo check
cargo test
```

## Roadmap

- Replace keyword matching with stronger detection policies
- Add persistent consent workflow (`grant`/`deny`)
- Add integration tests for JSON-RPC/MCP behavior
- Add configurable policy and audit paths

## License

MIT. See [LICENSE](LICENSE).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development and PR guidelines.
