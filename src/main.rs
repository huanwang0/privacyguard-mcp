//! PrivacyGuard MCP Server
//!
//! A minimal MCP server that routes sensitive data to local LLMs.

use anyhow::Result;
use privacyguard_mcp::PrivacyGuardServer;

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("🔒 PrivacyGuard MCP Server starting...");

    let server = PrivacyGuardServer::new();
    server.run_stdio().await?;

    Ok(())
}
