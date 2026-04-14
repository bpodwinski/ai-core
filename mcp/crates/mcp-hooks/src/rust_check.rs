use anyhow::Result;
use serde_json::Value;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn run() -> Result<()> {
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin)?;

    let data: Value = serde_json::from_str(&stdin).unwrap_or(Value::Null);
    let file_path = data
        .get("tool_input")
        .and_then(|v| v.get("file_path"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if !file_path.ends_with(".rs") {
        return Ok(());
    }

    let abs = PathBuf::from(file_path);
    let Some(cargo_root) = find_cargo_root(&abs) else {
        return Ok(());
    };

    // 1. cargo fmt — silent, always succeeds
    let _ = Command::new("cargo")
        .args(["fmt", "--all"])
        .current_dir(&cargo_root)
        .status();

    // 2. cargo clippy — capture output, forward on failure
    let out = Command::new("cargo")
        .args(["clippy", "--all-targets"])
        .current_dir(&cargo_root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    io::stdout().write_all(&out.stdout)?;
    io::stderr().write_all(&out.stderr)?;
    io::stdout().flush()?;
    io::stderr().flush()?;

    if !out.status.success() {
        std::process::exit(2);
    }
    Ok(())
}

fn find_cargo_root(file: &Path) -> Option<PathBuf> {
    let mut dir = file.parent()?.to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    loop {
        if dir.join("Cargo.toml").exists() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}
