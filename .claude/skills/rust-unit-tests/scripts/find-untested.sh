#!/usr/bin/env bash
# Scans a Rust project and reports functions/methods that likely lack unit tests.
# Usage: bash find-untested.sh [project-path]

PROJECT="${1:-.}"
SRC="$PROJECT/src"

if [ ! -d "$SRC" ]; then
  echo "ERROR: No src/ directory found at '$PROJECT'"
  exit 1
fi

echo "=== Cargo.toml(s) ==="
find "$PROJECT" -name "Cargo.toml" -not -path "*/target/*" 2>/dev/null

echo ""
echo "=== Rust source files ==="
find "$SRC" -name "*.rs" 2>/dev/null

echo ""
echo "=== Public functions (pub fn) ==="
grep -rn "pub fn " "$SRC" --include="*.rs" 2>/dev/null \
  | grep -v "mod tests" \
  | grep -v "^Binary"

echo ""
echo "=== Private functions with non-trivial logic (fn, with body) ==="
grep -rn "^\s*fn " "$SRC" --include="*.rs" 2>/dev/null \
  | grep -v "mod tests" \
  | grep -v "test_"

echo ""
echo "=== Files WITH existing test module ==="
grep -rl "#\[cfg(test)\]" "$SRC" --include="*.rs" 2>/dev/null

echo ""
echo "=== Files WITHOUT any test module ==="
for f in $(find "$SRC" -name "*.rs" 2>/dev/null); do
  if ! grep -q "#\[cfg(test)\]" "$f" 2>/dev/null; then
    echo "$f"
  fi
done

echo ""
echo "=== Existing test function names ==="
grep -rn "#\[test\]" "$SRC" --include="*.rs" -A1 2>/dev/null | grep "fn test_"
