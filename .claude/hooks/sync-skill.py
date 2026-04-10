#!/usr/bin/env python3
"""
PostToolUse hook — three responsibilities:
  1. Write/Edit in .claude/skills/  → mirror to .agents/skills/
  2. Write/Edit of CLAUDE.md        → copy as AGENTS.md in same directory
  3. Bash touching .claude/skills/  → prune orphans from .agents/skills/
"""
import json, sys, os, shutil

data = json.load(sys.stdin)
tool_name = data.get("tool_name", "")
tool_input = data.get("tool_input", {})

# Project root derived from this script's location:
# <project>/.claude/hooks/sync-skill.py
project_root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


def sync_file(file_path):
    normalized = file_path.replace("\\", "/")

    # 1. File inside .claude/skills/ → copy to .agents/skills/
    marker = "/.claude/skills/"
    if marker in normalized:
        suffix = normalized[normalized.index(marker) + len(marker):]
        dest = os.path.join(project_root, ".agents", "skills", *suffix.split("/"))
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        shutil.copy2(file_path, dest)
        print(f"[sync] skill -> {dest}")

    # 2. Any CLAUDE.md → copy as AGENTS.md alongside it
    if os.path.basename(normalized) == "CLAUDE.md":
        agents_path = os.path.join(os.path.dirname(file_path), "AGENTS.md")
        shutil.copy2(file_path, agents_path)
        print(f"[sync] AGENTS.md -> {agents_path}")


def prune_agents_skills():
    """Remove .agents/skills/ entries that no longer exist in .claude/skills/"""
    src_root = os.path.join(project_root, ".claude", "skills")
    dst_root = os.path.join(project_root, ".agents", "skills")
    if not os.path.isdir(dst_root):
        return
    for dirpath, dirs, files in os.walk(dst_root, topdown=False):
        for f in files:
            dst_file = os.path.join(dirpath, f)
            src_file = os.path.join(src_root, os.path.relpath(dst_file, dst_root))
            if not os.path.exists(src_file):
                os.remove(dst_file)
                print(f"[sync] pruned -> {dst_file}")
        # Remove now-empty directories
        try:
            os.rmdir(dirpath)
            print(f"[sync] removed dir -> {dirpath}")
        except OSError:
            pass  # not empty, that's fine


if tool_name in ("Write", "Edit"):
    file_path = tool_input.get("file_path", "")
    if file_path:
        sync_file(file_path)

elif tool_name == "Bash":
    command = tool_input.get("command", "")
    if ".claude/skills" in command or ".claude\\skills" in command:
        prune_agents_skills()
