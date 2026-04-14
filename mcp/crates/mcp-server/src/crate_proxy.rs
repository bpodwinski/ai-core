use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout};
use tokio::sync::Mutex;

/// Subprocess bridge to rust-docs-mcp (stdio JSON-RPC MCP protocol).
/// Spawned once at startup; tool calls are forwarded via stdin/stdout.
pub struct CrateProxy {
    stdin: Mutex<ChildStdin>,
    stdout: Mutex<BufReader<ChildStdout>>,
    next_id: Mutex<u64>,
}

impl CrateProxy {
    /// Spawn `rust-docs-mcp` as a persistent subprocess and perform the MCP handshake.
    pub async fn spawn() -> anyhow::Result<Arc<Self>> {
        let mut child = tokio::process::Command::new("rust-docs-mcp")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("no stdin"))?;
        let stdout = BufReader::new(
            child
                .stdout
                .take()
                .ok_or_else(|| anyhow::anyhow!("no stdout"))?,
        );

        // Let the subprocess run as a daemon — we hold its stdin/stdout handles.
        tokio::spawn(async move {
            let _ = child.wait().await;
        });

        let proxy = Arc::new(Self {
            stdin: Mutex::new(stdin),
            stdout: Mutex::new(stdout),
            next_id: Mutex::new(1),
        });

        proxy.handshake().await?;
        Ok(proxy)
    }

    async fn next_id(&self) -> u64 {
        let mut id = self.next_id.lock().await;
        let current = *id;
        *id += 1;
        current
    }

    /// Write one JSON-RPC message (newline-delimited).
    async fn send(&self, value: &serde_json::Value) -> anyhow::Result<()> {
        let mut stdin = self.stdin.lock().await;
        let msg = serde_json::to_string(value)? + "\n";
        stdin.write_all(msg.as_bytes()).await?;
        stdin.flush().await?;
        Ok(())
    }

    /// Read lines until we get a JSON-RPC response (has an `id` field).
    /// Notifications (no `id`) are skipped.
    async fn recv(&self) -> anyhow::Result<serde_json::Value> {
        let mut stdout = self.stdout.lock().await;
        loop {
            let mut line = String::new();
            let n = stdout.read_line(&mut line).await?;
            if n == 0 {
                anyhow::bail!("rust-docs-mcp subprocess closed stdout");
            }
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let val: serde_json::Value = serde_json::from_str(line)?;
            if val.get("id").is_some() {
                return Ok(val);
            }
            // Skip notifications
        }
    }

    /// MCP initialize handshake, then list available tools for diagnostics.
    async fn handshake(&self) -> anyhow::Result<()> {
        let id = self.next_id().await;
        self.send(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "mcp-docs-bridge", "version": "1.0" }
            }
        }))
        .await?;
        self.recv().await?; // consume initialize response

        // Notify the subprocess that initialization is done
        self.send(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }))
        .await?;

        // List available tools for diagnostics
        let list_id = self.next_id().await;
        self.send(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": list_id,
            "method": "tools/list"
        }))
        .await?;
        match self.recv().await {
            Ok(resp) => {
                if let Some(tools) = resp["result"]["tools"].as_array() {
                    for tool in tools {
                        let name = tool["name"].as_str().unwrap_or("?");
                        let props = &tool["inputSchema"]["properties"];
                        let required = &tool["inputSchema"]["required"];
                        tracing::info!(
                            "tool {:?} — required: {} — params: {}",
                            name,
                            required,
                            props,
                        );
                    }
                }
            }
            Err(e) => tracing::warn!("Could not list rust-docs-mcp tools: {e}"),
        }

        Ok(())
    }

    /// Forward a tool call to rust-docs-mcp and return the text content.
    pub async fn call_tool(&self, tool_name: &str, args: serde_json::Value) -> String {
        let id = self.next_id().await;
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/call",
            "params": { "name": tool_name, "arguments": args }
        });

        if let Err(e) = self.send(&req).await {
            return format!("Failed to send request to rust-docs-mcp: {e}");
        }

        match self.recv().await {
            Ok(resp) => {
                if let Some(err) = resp.get("error") {
                    return format!("rust-docs-mcp error: {err}");
                }
                resp["result"]["content"]
                    .as_array()
                    .and_then(|a| a.first())
                    .and_then(|c| c["text"].as_str())
                    .unwrap_or("(no response)")
                    .to_string()
            }
            Err(e) => format!("Failed to read response from rust-docs-mcp: {e}"),
        }
    }
}
