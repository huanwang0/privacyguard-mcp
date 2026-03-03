use crate::tools::{audit, ollama, privacy};
use anyhow::Result;
use serde_json::Value;

/// PrivacyGuard MCP Server
///
/// Implements MCP protocol with stdio transport for privacy-aware AI routing
pub struct PrivacyGuardServer {
    name: String,
    version: String,
}

impl PrivacyGuardServer {
    pub fn new() -> Self {
        Self {
            name: "privacyguard-mcp".to_string(),
            version: "0.1.0".to_string(),
        }
    }

    /// Handle MCP tool call
    pub async fn handle_call_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        match name {
            "analyze_privacy" => {
                let text = arguments["text"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'text' parameter"))?;
                let result = privacy::check_privacy(text);
                Ok(serde_json::to_value(result)?)
            }
            "route_to_local" => {
                let model = arguments["model"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'model' parameter"))?;
                let prompt = arguments["prompt"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing 'prompt' parameter"))?;
                let client = ollama::OllamaClient::new("http://127.0.0.1:11434");
                let response = client.generate(model, prompt).await?;
                Ok(serde_json::json!({
                    "response": response,
                    "model": model,
                    "routed_to": "local"
                }))
            }
            "get_audit_logs" => {
                let limit = arguments["limit"].as_u64().unwrap_or(10) as usize;
                let logs = audit::read_recent_events("audit.log", limit).await?;
                Ok(serde_json::json!({
                    "logs": logs,
                    "count": logs.len()
                }))
            }
            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        }
    }

    /// Run server with stdio transport
    pub async fn run_stdio(&self) -> Result<()> {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        eprintln!("🔒 PrivacyGuard MCP Server ready (stdio mode)");
        eprintln!("Waiting for MCP requests...");

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => match self.process_mcp_request(&line).await {
                    Ok(response) if !response.is_empty() => {
                        println!("{}", response);
                        stdout.flush().await?;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        let parse_error = self.error_response(
                            Value::Null,
                            -32700,
                            &format!("Parse error: {}", e),
                        )?;
                        println!("{}", parse_error);
                        stdout.flush().await?;
                    }
                },
                Err(e) => {
                    eprintln!("Error reading stdin: {}", e);
                }
            }
        }

        Ok(())
    }

    async fn process_mcp_request(&self, line: &str) -> Result<String> {
        // Simple JSON-RPC style handling
        let request: Value = serde_json::from_str(line.trim())?;

        let method = request["method"].as_str().unwrap_or("");
        let params = &request["params"];
        let id = request.get("id").cloned().unwrap_or(Value::Null);

        match method {
            "initialize" => {
                let result = serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": self.name,
                        "version": self.version
                    }
                });
                self.success_response(id, result)
            }
            "tools/list" => {
                let result = serde_json::json!({
                    "tools": [
                        {
                            "name": "analyze_privacy",
                            "description": "Analyze text for sensitive data (PII, PHI, secrets)",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "text": {"type": "string", "description": "Text to analyze"}
                                },
                                "required": ["text"]
                            }
                        },
                        {
                            "name": "route_to_local",
                            "description": "Route sensitive query to local Ollama LLM",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "model": {"type": "string"},
                                    "prompt": {"type": "string"}
                                },
                                "required": ["model", "prompt"]
                            }
                        },
                        {
                            "name": "get_audit_logs",
                            "description": "Retrieve recent privacy audit events",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "limit": {"type": "number", "default": 10}
                                }
                            }
                        }
                    ]
                });
                self.success_response(id, result)
            }
            "tools/call" => {
                let tool_name = params["name"].as_str().unwrap_or("");
                let args = &params["arguments"];
                if tool_name.is_empty() {
                    return self.error_response(id, -32602, "Invalid params: missing tool name");
                }

                let result = match self.handle_call_tool(tool_name, args.clone()).await {
                    Ok(result) => serde_json::json!({
                        "content": [{"type": "text", "text": result.to_string()}],
                        "isError": false
                    }),
                    Err(e) => serde_json::json!({
                        "content": [{"type": "text", "text": e.to_string()}],
                        "isError": true
                    }),
                };
                self.success_response(id, result)
            }
            "notifications/initialized" => {
                // No response needed
                Ok(String::new())
            }
            _ => self.error_response(id, -32601, &format!("Method not found: {}", method)),
        }
    }

    fn success_response(&self, id: Value, result: Value) -> Result<String> {
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result
        });

        Ok(serde_json::to_string(&response)?)
    }

    fn error_response(&self, id: Value, code: i64, message: &str) -> Result<String> {
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": code,
                "message": message
            }
        });

        Ok(serde_json::to_string(&response)?)
    }
}

impl Default for PrivacyGuardServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unknown_method_returns_jsonrpc_error() {
        let server = PrivacyGuardServer::new();
        let response = server
            .process_mcp_request(r#"{"jsonrpc":"2.0","id":7,"method":"nope","params":{}}"#)
            .await
            .expect("response should serialize");

        let parsed: Value = serde_json::from_str(&response).expect("valid json");
        assert_eq!(parsed["jsonrpc"], "2.0");
        assert_eq!(parsed["id"], 7);
        assert_eq!(parsed["error"]["code"], -32601);
        assert!(parsed["result"].is_null());
    }

    #[tokio::test]
    async fn tools_call_without_name_returns_invalid_params() {
        let server = PrivacyGuardServer::new();
        let response = server
            .process_mcp_request(
                r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"arguments":{}}}"#,
            )
            .await
            .expect("response should serialize");

        let parsed: Value = serde_json::from_str(&response).expect("valid json");
        assert_eq!(parsed["error"]["code"], -32602);
    }

    #[tokio::test]
    async fn initialized_notification_returns_no_response() {
        let server = PrivacyGuardServer::new();
        let response = server
            .process_mcp_request(
                r#"{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}"#,
            )
            .await
            .expect("method should be handled");

        assert!(response.is_empty());
    }
}
