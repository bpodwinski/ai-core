---
name: rust-clippy
description: Incrementally fix Clippy lints across a Cargo workspace. Use this skill whenever the user asks to fix clippy warnings, address lints, clean up Rust code quality, run a clippy pass, or reduce warnings. Also trigger when the user mentions "clippy bot", "rust-clippy", "lint pass", "fix warnings", "deny lints", or wants to systematically address clippy diagnostics. Works on any multi-crate Cargo workspace with a persistent branch strategy and incremental history tracking.
argument-hint: [crate-name or path — omit to scan the whole workspace]
---

# Rust Clippy

## Reference files

| File | Purpose |
|------|---------|
| [`template.md`](template.md) | Common clippy lint patterns with before/after fixes |
| [`docs/RUSTCLIPPY_HISTORY.md`](docs/RUSTCLIPPY_HISTORY.md) | Pass history — tracks lint categories addressed per session |

Fix Clippy lints incrementally across a Cargo workspace. One small, safe batch of lint fixes per run — no behavior changes, no speculative refactors.

## How It Works

Each run follows the same disciplined loop:

1. Read `docs/RUSTCLIPPY_HISTORY.md` to know what lint categories have already been addressed
2. Run `cargo clippy --message-format=json` and parse warnings by category
3. Pick ONE small, coherent batch (same lint or same module)
4. Fix the lints, validate, commit to the persistent branch
5. Append to history

---

## Step-by-Step Workflow

### 1. Read Clippy History

Before touching any file, read `docs/RUSTCLIPPY_HISTORY.md` (if it exists).

Use it to:

- Skip lint categories already fixed in past runs
- Avoid re-introducing lints that were already addressed
- Identify the most impactful remaining categories

### 2. Git Setup

Use a persistent branch named `rust-clippy`:

```bash
git checkout main
git pull --rebase
git checkout rust-clippy 2>/dev/null || git checkout -b rust-clippy
git rebase main
```

Never force push. Never create date-based branches.

### 3. Collect Clippy Diagnostics

Run clippy and capture structured output:

```bash
cargo clippy --workspace --all-targets --message-format=json 2>/dev/null \
  | grep '"level":"warning"' \
  | head -100
```

Or for a human-readable summary grouped by lint code:

```bash
cargo clippy --workspace --all-targets 2>&1 \
  | grep "^warning" \
  | sort | uniq -c | sort -rn \
  | head -30
```

Focus on:
- `clippy::unwrap_used` / `clippy::expect_used` (panic risk)
- `clippy::needless_pass_by_value` (performance)
- `clippy::redundant_closure` (readability)
- `clippy::match_wildcard_for_single_variants` (exhaustiveness)
- `clippy::too_many_arguments` (refactoring signal)

Skip entirely:
- `clippy::pedantic` unless the project opts in
- `clippy::restriction` lints unless specifically requested
- Lints inside `#[cfg(test)]` blocks unless they're genuine bugs

### 4. Select ONE Coherent Batch

Pick a single lint category in a single module or file. Keep the diff under **200 changed lines**. If a category has many occurrences, fix a subset and continue next run.

Prioritize:
- `deny`-level lints first (they block compilation)
- High-frequency lints that affect the most files
- Lints in library crates before binaries

### 5. Apply Fixes

Prefer `cargo clippy --fix` for mechanical fixes:

```bash
cargo clippy --fix --workspace --allow-dirty --allow-staged
```

For non-mechanical fixes, apply manually using the patterns in `template.md`.

**Rules:**
- **No behavior changes** — lint fixes only
- **No speculative refactors** — fix exactly what clippy flags
- **No new `#[allow(...)]` suppressions** unless the lint is a false positive with a comment explaining why
- If a fix would require non-trivial refactoring, suppress with `#[allow(clippy::lint_name)] // <reason>` and document in history

### 6. Validate

```bash
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets 2>&1 | grep "^warning\[clippy" | wc -l
cargo test --workspace
```

All checks must pass. The warning count must not increase.

### 7. Update Clippy History

Append (never rewrite) an entry to `docs/RUSTCLIPPY_HISTORY.md`:

```markdown
## YYYY-MM-DDTHH:MM:SSZ

- **Area**: `crate_name::module`
- **Lints fixed**: N
- **Categories**: `clippy::unwrap_used` (5), `clippy::needless_pass_by_value` (3)
- **Suppressions added**: 0
- **Remaining warnings**: 12
- **Validation**: `cargo test --workspace` — pass
```

---

## Hard Constraints

- **No behavior changes** — only lint fixes
- **No refactors** beyond what clippy requests
- **No new dependencies**
- **No weakening CI** — never lower `deny` to `warn` without discussion
- **Diffs under 200 lines** — stop early and continue next run if needed
- **No blanket `#![allow(clippy::all)]`** — fix or suppress individually
