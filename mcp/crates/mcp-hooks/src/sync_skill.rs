//! Post-edit hook that mirrors `CLAUDE.md` → `AGENTS.md` for Codex CLI.

use anyhow::{Context, Result};
use serde_json::Value;
use std::io::{self, Read};
use std::path::Path;

/// Mirror `CLAUDE.md` to `AGENTS.md` in the same directory after a Write/Edit
/// hook event targeting `CLAUDE.md`.
///
/// Reads a Claude Code `PostToolUse` JSON event from stdin. Does nothing if the
/// tool name is not `"Write"` or `"Edit"`, or if the target file is not named
/// `CLAUDE.md`.
///
/// # Errors
///
/// Returns an error if stdin cannot be read or if the file copy fails.
pub fn run() -> Result<()> {
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin)?;

    let data: Value = serde_json::from_str(&stdin).unwrap_or(Value::Null);
    let tool_name = data.get("tool_name").and_then(|v| v.as_str()).unwrap_or("");
    if tool_name != "Write" && tool_name != "Edit" {
        return Ok(());
    }

    let file_path = data
        .get("tool_input")
        .and_then(|v| v.get("file_path"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if file_path.is_empty() {
        return Ok(());
    }

    let path = Path::new(file_path);
    if path.file_name().and_then(|s| s.to_str()) != Some("CLAUDE.md") {
        return Ok(());
    }

    let agents = path.with_file_name("AGENTS.md");
    std::fs::copy(path, &agents)
        .with_context(|| format!("copying {} → {}", path.display(), agents.display()))?;
    println!("[sync] AGENTS.md -> {}", agents.display());
    Ok(())
}
