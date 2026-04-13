#!/usr/bin/env bash
# Runs cargo test and summarizes the output.
# Usage: bash run-tests.sh [project-path]

PROJECT="${1:-.}"

cd "$PROJECT" || { echo "ERROR: Cannot cd to '$PROJECT'"; exit 1; }

echo "=== Running: cargo test ==="
cargo test 2>&1
echo ""
echo "=== Exit code: $? ==="
