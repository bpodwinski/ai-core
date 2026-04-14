use anyhow::Result;
use clap::{Parser, Subcommand};

mod rust_check;
mod sync_skill;

#[derive(Parser)]
#[command(about = "Claude Code PostToolUse hooks for ai-core")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run cargo fmt + cargo clippy after a .rs file is edited
    RustCheck,
    /// Mirror CLAUDE.md → AGENTS.md after CLAUDE.md is written/edited
    SyncSkill,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::RustCheck => rust_check::run(),
        Cmd::SyncSkill => sync_skill::run(),
    }
}
