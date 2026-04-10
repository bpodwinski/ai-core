#!/usr/bin/env python3
"""
PostToolUse hook — sync .claude/skills/ → .agents/skills/
Triggered after Write or Edit tool calls.
"""
import json, sys, os, shutil

data = json.load(sys.stdin)
file_path = data.get("tool_input", {}).get("file_path", "")
if not file_path:
    sys.exit(0)

normalized = file_path.replace("\\", "/")
marker = "/.claude/skills/"

if marker not in normalized:
    sys.exit(0)

# Derive project root from this script's own location:
# script sits at <project>/.claude/hooks/sync-skill.py
project_root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

suffix = normalized[normalized.index(marker) + len(marker):]  # ex: rust-unit-tests/SKILL.md

dest = os.path.join(project_root, ".agents", "skills", *suffix.split("/"))
os.makedirs(os.path.dirname(dest), exist_ok=True)
shutil.copy2(file_path, dest)
print(f"[sync-skill] synced -> {dest}")
