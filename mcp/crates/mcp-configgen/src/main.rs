//! Config generator binary.
//!
//! Reads `servers-manifest.json` and emits `dist/claude-mcp.json` (Claude Code
//! client config) and `dist/codex-config.toml` (Codex CLI config).

use anyhow::{Context, Result};
use clap::Parser;
use mcp_common::{DocSourceEntry, ExternalServer, Manifest, StdioServer};
use serde_json::{json, Map, Value};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    about = "Generate MCP client configs (claude-mcp.json + codex-config.toml) from servers-manifest.json"
)]
struct Args {
    /// Path to servers-manifest.json
    #[arg(long, default_value = "servers-manifest.json")]
    manifest: PathBuf,

    /// Output directory for generated files
    #[arg(long, default_value = "../dist")]
    out_dir: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let manifest = Manifest::load(&args.manifest)?;

    std::fs::create_dir_all(&args.out_dir)
        .with_context(|| format!("creating {}", args.out_dir.display()))?;

    let claude_path = args.out_dir.join("claude-mcp.json");
    let codex_path = args.out_dir.join("codex-config.toml");

    let claude_json = build_claude_config(&manifest);
    std::fs::write(&claude_path, &claude_json)?;
    println!("Written: {}", claude_path.display());

    let codex_toml = build_codex_toml(&manifest);
    std::fs::write(&codex_path, &codex_toml)?;
    println!("Written: {}", codex_path.display());

    let total = 1 + manifest.external.len() + manifest.stdio.len();
    println!(
        "\n1 multi-tenant ({} doc sources) + {} external + {} stdio = {} servers",
        manifest.doc_sources.len(),
        manifest.external.len(),
        manifest.stdio.len(),
        total
    );

    Ok(())
}

fn build_claude_config(manifest: &Manifest) -> String {
    let mut servers = Map::new();

    servers.insert(
        manifest.mcp_name.clone(),
        json!({
            "type": "http",
            "url": format!("{}{}", manifest.base_url, manifest.mcp_endpoint),
        }),
    );

    for s in &manifest.external {
        servers.insert(s.name.clone(), external_entry(s));
    }

    for s in &manifest.stdio {
        servers.insert(s.name.clone(), stdio_entry(s));
    }

    let root = json!({ "mcpServers": Value::Object(servers) });
    let mut out = serde_json::to_string_pretty(&root).expect("serialize");
    out.push('\n');
    out
}

fn external_entry(s: &ExternalServer) -> Value {
    match &s.bearer_token_env_var {
        Some(var) => json!({
            "type": "http",
            "url": s.url,
            "headers": { "Authorization": format!("Bearer ${{{}}}", var) },
        }),
        None => json!({
            "type": "http",
            "url": s.url,
        }),
    }
}

fn stdio_entry(s: &StdioServer) -> Value {
    let mut obj = Map::new();
    obj.insert("type".into(), Value::String("stdio".into()));
    obj.insert("command".into(), Value::String(s.command.clone()));
    obj.insert(
        "args".into(),
        Value::Array(s.args.iter().cloned().map(Value::String).collect()),
    );
    if let Some(env) = &s.env {
        let mut env_map = Map::new();
        for (k, v) in env {
            env_map.insert(k.clone(), Value::String(v.clone()));
        }
        obj.insert("env".into(), Value::Object(env_map));
    }
    Value::Object(obj)
}

fn build_codex_toml(manifest: &Manifest) -> String {
    let categories: Vec<&str> = manifest
        .doc_sources
        .iter()
        .map(|s: &DocSourceEntry| s.name.as_str())
        .collect();

    let header_lines = [
        "# Codex CLI — MCP servers configuration".to_string(),
        "# Source : https://github.com/bpodwinski/ai-core/releases/latest/download/codex-config.toml".to_string(),
        "#".to_string(),
        "# Installation :".to_string(),
        "#   curl -fsSL https://github.com/bpodwinski/ai-core/releases/latest/download/codex-config.toml \\".to_string(),
        "#        >> ~/.codex/config.toml".to_string(),
        "#".to_string(),
        "# Auth : codex mcp login docs".to_string(),
        "".to_string(),
        format!("[mcp_servers.{}]", manifest.mcp_name),
        format!("url = \"{}{}\"", manifest.base_url, manifest.mcp_endpoint),
        format!(
            "# Multi-tenant docs server — categories: {}",
            categories.join(", ")
        ),
        "".to_string(),
    ];
    let header = header_lines.join("\n");

    let external_header = if !manifest.external.is_empty() {
        "# ── External MCP servers ─────────────────────────────────────────────────────\n\n"
            .to_string()
    } else {
        String::new()
    };

    let external_blocks: Vec<String> = manifest
        .external
        .iter()
        .map(|s| {
            let mut lines = vec![
                format!("[mcp_servers.{}]", s.name),
                format!("url = \"{}\"", s.url),
            ];
            if let Some(var) = &s.bearer_token_env_var {
                lines.push(format!("bearer_token_env_var = \"{}\"", var));
            }
            lines.push(format!("# description = \"{}\"", s.description));
            lines.push(String::new());
            lines.join("\n")
        })
        .collect();

    format!(
        "{}{}{}",
        header,
        external_header,
        external_blocks.join("\n")
    )
}
