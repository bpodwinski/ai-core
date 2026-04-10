---
name: rust-unit-tests
description: Scans a Rust project to identify functions/methods missing unit tests, then generates them. Usage: /rust-unit-tests [path] — path is optional, defaults to the current working directory.
argument-hint: [path/optional]
allowed-tools: Glob Grep Read Edit Write Bash TodoWrite
---

# Rust Unit Test Generation

## Live project scan

```!
bash "${CLAUDE_SKILL_DIR}/scripts/find-untested.sh" "$ARGUMENTS"
```

---

## Your task

Using the scan output above, generate unit tests for all untested functions in this Rust project.

### Supporting files — read these before writing any tests

- Test block format and naming rules → [template.md](template.md)
- Example: pure function → [examples/pure-function.md](examples/pure-function.md)
- Example: Result<T, E> → [examples/result-type.md](examples/result-type.md)
- Example: struct methods → [examples/struct-methods.md](examples/struct-methods.md)
- Expected report format → [examples/report.md](examples/report.md)

### Process

1. **Read** `template.md` and the relevant examples for the function types present in this project.
2. **Prioritize** functions in this order:
   - Public functions with no test module in their file
   - Public functions whose file has a test module but no test covering them
   - Complex private functions
3. **Skip** without comment: `main()`, trivial getters/setters, derived trait impls (`Default`, `Clone`, etc.), framework init functions.
4. **Skip with TODO**: functions that require external I/O, a database, or network — add `// TODO: mock <Trait> to test <fn>` in the source file.
5. **Write** tests directly into each source file following `template.md`.
   - If `#[cfg(test)] mod tests` already exists → add inside it.
   - If not → append the full block at the end of the file.
6. After all edits, **run the tests**:

```!
bash "${CLAUDE_SKILL_DIR}/scripts/run-tests.sh" "$ARGUMENTS"
```

7. Fix any compilation errors in the generated tests.
8. **Output the final report** following the format in `examples/report.md`.
