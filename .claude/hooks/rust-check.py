#!/usr/bin/env python3
"""
PostToolUse hook — runs cargo fmt + cargo clippy after any Edit/Write to a .rs file.
Exits with 2 if clippy reports errors, which triggers an asyncRewake notification.
"""
import json
import os
import subprocess
import sys

data = json.load(sys.stdin)
file_path = (data.get("tool_input") or {}).get("file_path", "")

# Only process Rust source files
if not file_path.endswith(".rs"):
    sys.exit(0)

# Normalize path (handles Windows backslashes)
file_path = os.path.normpath(file_path)

# Walk up the directory tree to find the nearest Cargo.toml
search_dir = os.path.dirname(os.path.abspath(file_path))
cargo_root = None
while True:
    if os.path.exists(os.path.join(search_dir, "Cargo.toml")):
        cargo_root = search_dir
        break
    parent = os.path.dirname(search_dir)
    if parent == search_dir:  # reached filesystem root
        break
    search_dir = parent

if cargo_root is None:
    sys.exit(0)

# 1. Format — always silent-succeeds
subprocess.run(["cargo", "fmt", "--all"], cwd=cargo_root)

# 2. Clippy — capture output so it can be forwarded on failure
r = subprocess.run(
    ["cargo", "clippy", "--all-targets"],
    cwd=cargo_root,
    capture_output=True,
    text=True,
)
# Print to terminal regardless
sys.stdout.write(r.stdout)
sys.stderr.write(r.stderr)
sys.stdout.flush()
sys.stderr.flush()

# Exit 2 triggers asyncRewake — Claude sees clippy output as a system-reminder
if r.returncode != 0:
    sys.exit(2)
