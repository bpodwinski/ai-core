#!/usr/bin/env bash
# Install Claude Code + Codex CLI configs from the latest GitHub release.
# Also installs github-mcp-server binary (no Docker required).
# Usage (from project root):
#   bash install.sh
#   bash install.sh --project-dir /path/to/project
set -euo pipefail

REPO="bpodwinski/ai-core"
GH_MCP_REPO="github/github-mcp-server"
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

# ── 1. Claude Code + Codex configs ───────────────────────────────────────────
echo "Downloading latest configs from $REPO..."
curl -fsSL "$ZIP_URL" -o "$ZIP"

echo "Installing project configs → $PROJECT_DIR"
unzip -o "$ZIP" ".mcp.json" -d "$PROJECT_DIR"
unzip -o "$ZIP" ".claude/settings.json" -d "$PROJECT_DIR"

echo "Installing Codex config → ~/.codex/config.toml"
mkdir -p "$HOME/.codex"
unzip -o "$ZIP" ".codex/config.toml" -d "$HOME"

# ── 2. github-mcp-server binary ──────────────────────────────────────────────
echo ""
echo "Installing github-mcp-server..."

BIN_DIR="$HOME/.claude/bin"
mkdir -p "$BIN_DIR"

# Detect OS + arch
OS="$(uname -s)"
ARCH="$(uname -m)"
case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64)  ASSET="github-mcp-server_Linux_x86_64.tar.gz" ;;
      aarch64) ASSET="github-mcp-server_Linux_arm64.tar.gz" ;;
      i386)    ASSET="github-mcp-server_Linux_i386.tar.gz" ;;
      *) echo "Unsupported arch: $ARCH"; exit 1 ;;
    esac ;;
  Darwin)
    case "$ARCH" in
      arm64)   ASSET="github-mcp-server_Darwin_arm64.tar.gz" ;;
      x86_64)  ASSET="github-mcp-server_Darwin_x86_64.tar.gz" ;;
      *) echo "Unsupported arch: $ARCH"; exit 1 ;;
    esac ;;
  MINGW*|MSYS*|CYGWIN*)
    case "$ARCH" in
      x86_64)  ASSET="github-mcp-server_Windows_x86_64.zip" ;;
      aarch64) ASSET="github-mcp-server_Windows_arm64.zip" ;;
      *) echo "Unsupported arch: $ARCH"; exit 1 ;;
    esac ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

GH_MCP_URL="https://github.com/${GH_MCP_REPO}/releases/latest/download/${ASSET}"
GH_MCP_ARCHIVE="$TMP/$ASSET"

curl -fsSL "$GH_MCP_URL" -o "$GH_MCP_ARCHIVE"

if [[ "$ASSET" == *.zip ]]; then
  unzip -o "$GH_MCP_ARCHIVE" "github-mcp-server.exe" -d "$BIN_DIR"
else
  tar -xz -f "$GH_MCP_ARCHIVE" -C "$BIN_DIR" github-mcp-server
  chmod +x "$BIN_DIR/github-mcp-server"
fi

echo "github-mcp-server installed → $BIN_DIR"

# ── 3. mcp-hooks binary (Claude Code hooks for cargo fmt/clippy + AGENTS.md sync) ─
echo ""
echo "Installing mcp-hooks..."
case "$OS" in
  Linux)  MCP_HOOKS_ASSET="mcp-hooks-linux-x86_64"  ;;
  Darwin) MCP_HOOKS_ASSET="mcp-hooks-macos-arm64"   ;;
  MINGW*|MSYS*|CYGWIN*) MCP_HOOKS_ASSET="mcp-hooks-windows-x86_64.exe" ;;
esac
MCP_HOOKS_URL="https://github.com/${REPO}/releases/latest/download/${MCP_HOOKS_ASSET}"
case "$OS" in
  MINGW*|MSYS*|CYGWIN*) MCP_HOOKS_DEST="$BIN_DIR/mcp-hooks.exe" ;;
  *) MCP_HOOKS_DEST="$BIN_DIR/mcp-hooks" ;;
esac
curl -fsSL "$MCP_HOOKS_URL" -o "$MCP_HOOKS_DEST"
chmod +x "$MCP_HOOKS_DEST" 2>/dev/null || true
echo "mcp-hooks installed → $MCP_HOOKS_DEST"

rm -rf "$TMP"
echo ""
echo "Done. Set GITHUB_TOKEN in your environment or in ~/.claude/settings.json."
