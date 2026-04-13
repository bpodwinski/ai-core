#!/usr/bin/env bash
# Integration tests for cache_crate with features parameter
# Requires: mcp-docs container running and healthy
set -euo pipefail

CONTAINER="mcp-docs"
CRATE="serde"
VERSION="1.0.219"
PASS=0
FAIL=0
TOTAL=3
MSG_ID=0

# ── Helpers ──────────────────────────────────────────────────────────────────

next_id() { MSG_ID=$((MSG_ID + 1)); echo $MSG_ID; }

mcp_raw() {
  docker exec "$CONTAINER" curl -s -X POST http://localhost:80/mcp \
    -H "Content-Type: application/json" \
    -H "Accept: application/json, text/event-stream" \
    -H "Mcp-Session-Id: $SID" \
    -d "$1" 2>&1
}

mcp_init() {
  local resp
  resp=$(docker exec "$CONTAINER" curl -s -i -X POST http://localhost:80/mcp \
    -H "Content-Type: application/json" \
    -H "Accept: application/json, text/event-stream" \
    -d "{\"jsonrpc\":\"2.0\",\"id\":$(next_id),\"method\":\"initialize\",\"params\":{\"protocolVersion\":\"2025-03-26\",\"capabilities\":{},\"clientInfo\":{\"name\":\"test\",\"version\":\"1.0\"}}}" 2>&1)
  SID=$(echo "$resp" | grep -i "mcp-session-id" | tr -d '\r' | awk '{print $2}')
  # Send initialized notification
  mcp_raw "{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}" > /dev/null
  sleep 1
}

mcp_call() {
  local tool="$1" args="$2"
  local id
  id=$(next_id)
  mcp_raw "{\"jsonrpc\":\"2.0\",\"id\":$id,\"method\":\"tools/call\",\"params\":{\"name\":\"$tool\",\"arguments\":$args}}" \
    | grep "^data: {" | tail -1 | sed 's/^data: //'
}

wait_doc_generated() {
  local crate="$1" version="$2" timeout="${3:-120}" elapsed=0
  while [ $elapsed -lt $timeout ]; do
    local resp
    resp=$(mcp_call "list_cached_crates" "{}")
    # Inner JSON keys are double-escaped in the SSE envelope ("doc_generated" → \"doc_generated\")
    # so we match substrings without surrounding quotes to avoid backslash mismatch
    if echo "$resp" | grep -q "doc_generated.*true"; then
      if echo "$resp" | grep -q "$version"; then
        return 0
      fi
    fi
    sleep 5
    elapsed=$((elapsed + 5))
  done
  return 1
}

remove_crate() {
  mcp_call "remove_crate" "{\"crate_name\":\"$1\",\"version\":\"$2\"}" > /dev/null 2>&1 || true
}

report() {
  local num="$1" status="$2" desc="$3"
  if [ "$status" = "PASS" ]; then
    PASS=$((PASS + 1))
    echo "[$num/$TOTAL] PASS: $desc"
  else
    FAIL=$((FAIL + 1))
    echo "[$num/$TOTAL] FAIL: $desc"
  fi
}

# ── Pre-checks ───────────────────────────────────────────────────────────────

echo "Waiting for $CONTAINER to be healthy..."
for i in $(seq 1 30); do
  if docker exec "$CONTAINER" curl -sf http://localhost:80/health > /dev/null 2>&1; then
    break
  fi
  if [ "$i" -eq 30 ]; then
    echo "ERROR: $CONTAINER not healthy after 30s"
    exit 1
  fi
  sleep 1
done
echo "Container healthy. Initializing MCP session..."

SID=""
mcp_init
echo "Session: $SID"
echo ""

# Clean up any leftover cache from previous runs
remove_crate "$CRATE" "$VERSION"

# ── Test 1: cache_crate with features: ["derive"] ───────────────────────────

echo "--- Test 1: features: [\"derive\"] ---"
resp=$(mcp_call "cache_crate" "{\"crate_name\":\"$CRATE\",\"source_type\":\"cratesio\",\"version\":\"$VERSION\",\"features\":[\"derive\"]}")
if echo "$resp" | grep -q "in_progress"; then
  if wait_doc_generated "$CRATE" "$VERSION" 180; then
    report 1 "PASS" "cache_crate with features: [\"derive\"]"
  else
    report 1 "FAIL" "cache_crate with features: [\"derive\"] — doc not generated within timeout"
  fi
else
  report 1 "FAIL" "cache_crate with features: [\"derive\"] — task not started"
fi
remove_crate "$CRATE" "$VERSION"

# ── Test 2: cache_crate with features: [] ────────────────────────────────────

echo "--- Test 2: features: [] ---"
resp=$(mcp_call "cache_crate" "{\"crate_name\":\"$CRATE\",\"source_type\":\"cratesio\",\"version\":\"$VERSION\",\"features\":[]}")
if echo "$resp" | grep -q "in_progress"; then
  if wait_doc_generated "$CRATE" "$VERSION" 180; then
    report 2 "PASS" "cache_crate with features: []"
  else
    report 2 "FAIL" "cache_crate with features: [] — doc not generated within timeout"
  fi
else
  report 2 "FAIL" "cache_crate with features: [] — task not started"
fi
remove_crate "$CRATE" "$VERSION"

# ── Test 3: cache_crate without features (fallback) ─────────────────────────

echo "--- Test 3: no features (fallback) ---"
resp=$(mcp_call "cache_crate" "{\"crate_name\":\"$CRATE\",\"source_type\":\"cratesio\",\"version\":\"$VERSION\"}")
if echo "$resp" | grep -q "in_progress"; then
  if wait_doc_generated "$CRATE" "$VERSION" 180; then
    report 3 "PASS" "cache_crate without features (fallback)"
  else
    report 3 "FAIL" "cache_crate without features (fallback) — doc not generated within timeout"
  fi
else
  report 3 "FAIL" "cache_crate without features (fallback) — task not started"
fi
remove_crate "$CRATE" "$VERSION"

# ── Summary ──────────────────────────────────────────────────────────────────

echo ""
if [ $FAIL -eq 0 ]; then
  echo "All $TOTAL tests passed."
  exit 0
else
  echo "$FAIL/$TOTAL tests failed."
  exit 1
fi
