#!/bin/bash
# PrivacyGuard MCP - stdio JSON-RPC demo

set -euo pipefail

BIN="./target/release/privacyguard-mcp"

if [ ! -x "$BIN" ]; then
  echo "Binary not found: $BIN"
  echo "Run: cargo build --release"
  exit 1
fi

echo "PrivacyGuard MCP stdio demo"
echo

cat <<'JSON' | "$BIN"
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"analyze_privacy","arguments":{"text":"What is the weather today?"}}}
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"analyze_privacy","arguments":{"text":"My password is secret123"}}}
{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"get_audit_logs","arguments":{"limit":5}}}
JSON
