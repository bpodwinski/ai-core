# Example — Final Report

This is what the report you produce at the end of the skill must look like.

---

## Test Generation Report

| File | Functions tested | Tests created |
|------|-----------------|---------------|
| `src/math.rs` | `clamp`, `percentage` | 8 |
| `src/parser.rs` | `parse_port` | 9 |
| `src/config.rs` | `Config::new`, `Config::address`, `Config::is_valid`, `Config::with_max_connections` | 7 |

**Total: 24 tests created across 3 files**

### Skipped (with reason)

| Item | Reason |
|------|--------|
| `src/main.rs::main` | Entry point — not unit-testable |
| `src/db.rs::connect` | Requires live database connection — no mock available. `// TODO: mock DbPool to test connect` added in source. |
| `src/config.rs::Config::default` | Derived by macro — no logic to test |

### Next steps

```bash
cargo test
```

All generated tests are in `#[cfg(test)]` blocks and will not affect the release binary.
