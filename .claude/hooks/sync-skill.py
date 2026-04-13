#!/usr/bin/env python3
"""
PostToolUse hook — sync CLAUDE.md → AGENTS.md

When a CLAUDE.md file is created or edited, automatically creates/updates
an AGENTS.md copy in the same directory (for Codex CLI compatibility).
"""
import json, sys, os, shutil

data = json.load(sys.stdin)
tool_name = data.get("tool_name", "")
tool_input = data.get("tool_input", {})

if tool_name not in ("Write", "Edit"):
    sys.exit(0)

file_path = tool_input.get("file_path", "")
if not file_path:
    sys.exit(0)

normalized = file_path.replace("\\", "/")
if os.path.basename(normalized) != "CLAUDE.md":
    sys.exit(0)

agents_path = os.path.join(os.path.dirname(file_path), "AGENTS.md")
shutil.copy2(file_path, agents_path)
print(f"[sync] AGENTS.md -> {agents_path}")
