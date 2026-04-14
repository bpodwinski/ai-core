#!/usr/bin/env bash
# Install Claude Code + Codex CLI configs from the latest GitHub release.
# Usage (from project root):
#   bash install.sh
#   bash install.sh --project-dir /path/to/project   # install .mcp.json + .claude/ elsewhere
set -euo pipefail

REPO="bpodwinski/ai-core"
ZIP_URL="https://github.com/${REPO}/releases/latest/download/claude-config.zip"
TMP=$(mktemp -d)
ZIP="$TMP/claude-config.zip"

# Resolve target project dir (default: current directory)
PROJECT_DIR="$PWD"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --project-dir) PROJECT_DIR="$2"; shift 2 ;;
    *) echo "Unknown option: $1"; exit 1 ;;
  esac
done

echo "Downloading latest configs from $REPO..."
curl -fsSL "$ZIP_URL" -o "$ZIP"

echo "Installing project configs → $PROJECT_DIR"
unzip -o "$ZIP" ".mcp.json" -d "$PROJECT_DIR"
unzip -o "$ZIP" ".claude/*" -d "$PROJECT_DIR"

echo "Installing Codex config → ~/.codex/config.toml"
mkdir -p "$HOME/.codex"
unzip -o "$ZIP" ".codex/config.toml" -d "$HOME"

rm -rf "$TMP"
echo "Done."
