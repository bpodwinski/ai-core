---
name: rust-doc
description: Incrementally add missing Rustdoc (///) for undocumented public Rust items across a Cargo workspace. Use this skill whenever the user asks to document Rust code, add missing rustdoc, improve documentation coverage, run a doc audit, or maintain documentation hygiene on a Rust project. Also trigger when the user mentions "rustdoc bot", "rust-doc", "documentation pass", "doc coverage", "undocumented pub items", or wants to systematically add /// comments to public API surfaces. Works on any multi-crate Cargo workspace with a persistent branch strategy and incremental history tracking.
argument-hint: [crate-name or path — omit to scan the whole workspace]
---

# Rust Doc

## Reference files

| File | Purpose |
|------|---------|
| [`template.md`](template.md) | Copy-paste templates for every Rust item kind (struct, enum, trait, fn, error, …) |
| [`examples/01_config_before.rs`](examples/01_config_before.rs) | Input example — struct + builder pattern, undocumented |
| [`examples/01_config_after.rs`](examples/01_config_after.rs) | Expected output — struct + builder pattern fully documented |
| [`examples/02_error_before.rs`](examples/02_error_before.rs) | Input example — error enum + fallible functions, undocumented |
| [`examples/02_error_after.rs`](examples/02_error_after.rs) | Expected output — `# Errors` sections and per-variant docs |
| [`examples/03_async_client_before.rs`](examples/03_async_client_before.rs) | Input example — async trait + impl, undocumented |
| [`examples/03_async_client_after.rs`](examples/03_async_client_after.rs) | Expected output — async trait with `# Contract` and `# Errors` |

Incrementally add missing `///` Rustdoc for undocumented public Rust items across a Cargo workspace. One small, safe documentation improvement per run — no behavior changes, no refactors.

## How It Works

Each run follows the same disciplined loop:

1. Read `docs/RUSTDOC_HISTORY.md` to know what's already been documented
2. Scan the workspace for undocumented `pub` items
3. Pick ONE small, coherent batch to document
4. Add `///` comments, validate, commit to the persistent branch
5. Append to history

This keeps diffs small, reviewable, and conflict-free.

---

## Step-by-Step Workflow

### 1. Read Documentation History

Before touching any file, read `docs/RUSTDOC_HISTORY.md` (if it exists).

Use it to:

- Skip modules/files already fully documented
- Avoid re-documenting items touched in recent runs
- Identify areas with the most remaining gaps

If the file doesn't exist yet, that's fine — create it after the first commit.

### 2. Git Setup

Use a persistent branch named `rust-doc`:

```bash
git checkout main
git pull --rebase
git checkout rust-doc 2>/dev/null || git checkout -b rust-doc
git rebase main
```

Never force push. Never create date-based branches. Never auto-merge or auto-PR.

### 3. Scan for Undocumented Public Items

Search Rust source files in the workspace for public items missing `///`:

Target directories: scan all crate roots in the workspace. Typical locations include `src/`, and any crate directories listed in `Cargo.toml` workspace members.

Skip entirely:

- `target/`, `.git/`, generated code, migrations, vendored/third-party code, build scripts (`build.rs`)
- Files and modules inside `#[cfg(test)]` blocks or `tests/` directories
- Items marked `#[doc(hidden)]` — they're intentionally hidden from docs

Target items:

- `pub struct`, `pub enum`, `pub trait`, `pub fn`, `pub type`, `pub mod`
- **Public methods in `impl` blocks** — methods declared `pub fn` inside an `impl MyStruct` (not just trait impls) are part of the public API and need `///` too

Ignore:

- `pub(crate)` unless the user explicitly requests it
- `pub use` re-exports — document at the **original definition site only**, not at the re-export. If the re-export adds semantic context (e.g. a facade module), a one-liner `///` on the `pub use` is acceptable but never duplicate the original doc.

A quick way to find candidates:

```bash
# List workspace crates, then scan for undocumented pub items
find . -name "*.rs" -not -path "*/target/*" -not -path "*/.git/*" -not -path "*/tests/*" \
  | xargs grep -ln "^pub \|^    pub " 2>/dev/null | head -40
```

Then for each file, check every `pub` item for a preceding `///` line. Also verify the item isn't inside a `#[cfg(test)]` block or marked `#[doc(hidden)]`.

#### Optional: measure coverage with `missing_docs`

To get a quick picture of how many items lack documentation, temporarily enable the lint:

```bash
RUSTFLAGS="-W missing_docs" cargo check --workspace 2>&1 | grep "missing documentation" | wc -l
```

This gives a count of all undocumented public items. Useful to track progress across runs — log the count in the history file. Don't commit this flag into the codebase; it's a measurement tool only.

### 4. Select ONE Coherent Batch

Pick a single module or a small group of related items (same file or tightly coupled files). Keep the diff under **300 changed lines**. If a module has too many undocumented items, document a subset and continue next run.

Prioritize:

- Public API surface (traits, structs, top-level functions)
- Library crates (`lib.rs`) before binary crates (`main.rs`)
- Items that appear in cross-crate boundaries

### 5. Write the Rustdoc

Style rules — follow these strictly:

- **US English only**
- **1–3 lines per item** — concise, no novels
- **Imperative verbs**: "Return…", "Build…", "Load…", "Persist…", "Extract…", "Validate…"
- **Explain intent/why**, not what's obvious from the signature
- **No speculation** — if context is unclear, keep it generic and safe
- **Never rewrite existing `///`** — only add where missing

Good examples:

```rust
/// Build a connection pool from the provided configuration to reuse across requests.
pub fn build_pool(config: &Config) -> Pool { ... }

/// Serialize the entity to JSON and write it to the output stream.
pub fn write_json<T: Serialize>(entity: &T, writer: &mut impl Write) -> Result<()> { ... }
```

Bad examples:

```rust
/// This function does stuff.     // too vague
/// Adds 1 to x.                  // restates the obvious
/// This is a very important...   // multi-paragraph essay
```

#### Doc-tests (examples in rustdoc)

For key public functions where usage isn't obvious, add a short `# Examples` block with a runnable doc-test. Keep it minimal — the goal is to show typical usage, not to be an integration test.

````rust
/// Parse a duration string like `"3h30m"` into a [`Duration`].
///
/// # Examples
///
/// ```
/// use my_crate::parse_duration;
///
/// let d = parse_duration("2h15m").unwrap();
/// assert_eq!(d.as_secs(), 8100);
/// ```
pub fn parse_duration(input: &str) -> Result<Duration> { ... }
````

Rules for doc-tests:

- Only add them to functions where the signature alone doesn't make usage clear
- Keep the example under 10 lines
- The example must compile and pass (`cargo test --doc`)
- Use `# ` prefix to hide boilerplate lines (imports, setup) when they add noise
- Don't add doc-tests to structs, enums, or traits — a description is enough
- If you're not confident the example compiles, skip it — a wrong doc-test is worse than none

### 6. Validate

Run all checks before committing — no exceptions:

```bash
cargo fmt --all
cargo doc --workspace --document-private-items --no-deps 2>&1 | grep -E "^error" && echo "DOC ERRORS" || echo "Docs OK"
cargo test --workspace --doc
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

`cargo doc` catches broken intra-doc links, malformed code blocks, and missing items referenced in docs. `cargo test --workspace --doc` runs all doc-tests (examples in `///` blocks). Both must pass.

If any check fails:

- Revert the problematic edits
- Do NOT weaken the checks
- Append a failure audit entry to the history
- Only commit if the branch is in a coherent state

### 7. Update Documentation History

Append (never rewrite) an entry to `docs/RUSTDOC_HISTORY.md`:

```markdown
## YYYY-MM-DDTHH:MM:SSZ

- **Area**: `crate_name::module::submodule`
- **Items documented**: 5
- **Coverage**: 142/187 public items documented (75.9%)
- **Summary**:
  - Added rustdoc for `MyStruct`, `MyEnum`, `ErrorKind`
  - Added rustdoc for `MyStruct::new()` and `MyStruct::validate()`
- **Validation**: pass
```

If no undocumented items were found:

```markdown
## YYYY-MM-DDTHH:MM:SSZ

- **Area**: full workspace audit
- **Items documented**: 0
- **Coverage**: 187/187 public items documented (100%)
- **Summary**: No undocumented public items found.
- **Validation**: n/a
```

Get the coverage count via:

```bash
TOTAL=$(RUSTFLAGS="-W missing_docs" cargo check --workspace 2>&1 | grep -c "missing documentation")
echo "$TOTAL undocumented public items remaining"
```

Log both the "items documented this run" and the "remaining total" so progress is trackable across runs.

---

## Hard Constraints

These are non-negotiable:

- **No behavior changes** — documentation only
- **No refactors** — don't rename, restructure, or "improve" code
- **No public API changes** — signatures stay untouched
- **No new dependencies**
- **No architecture boundary violations** — respect the existing module structure
- **No inline comments inside function bodies** — only `///` above items
- **No rewriting existing Rustdoc** — add, never modify
- **Diffs under 300 lines** — stop early and continue next run if needed

---

## Safety Philosophy

This agent is conservative by design:

- Prefer the smallest safe improvement
- Don't touch too many modules at once
- Avoid formatting-only noise (cargo fmt changes don't count toward the 300-line cap, but keep them minimal)
- Avoid speculative documentation — if you're not sure what something does, keep the doc generic
- Learn from history to avoid duplication
- When in doubt, skip and move on
