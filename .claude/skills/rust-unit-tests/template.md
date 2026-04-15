# Test Block Template

Use this template when writing unit tests. Always place tests at the bottom of the source file.

## Basic structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ── Pure function ──────────────────────────────────────────────────────────

    #[test]
    fn test_<function>_<scenario>() {
        // Arrange
        let input = <value>;

        // Act
        let result = <function>(input);

        // Assert
        assert_eq!(result, <expected>);
    }

    // ── Result<T, E> ───────────────────────────────────────────────────────────

    #[test]
    fn test_<function>_returns_ok_when_<condition>() {
        let result = <function>(<valid_input>);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), <expected>);
    }

    #[test]
    fn test_<function>_returns_err_when_<condition>() {
        let result = <function>(<invalid_input>);
        assert!(result.is_err());
    }

    // ── Option<T> ──────────────────────────────────────────────────────────────

    #[test]
    fn test_<function>_returns_some_when_<condition>() {
        let result = <function>(<input>);
        assert_eq!(result, Some(<expected>));
    }

    #[test]
    fn test_<function>_returns_none_when_<condition>() {
        let result = <function>(<input>);
        assert!(result.is_none());
    }

    // ── Struct method ──────────────────────────────────────────────────────────

    #[test]
    fn test_<Type>_<method>_<scenario>() {
        // Arrange — build the minimal struct needed
        let subject = <Type> {
            field: <value>,
            ..Default::default()  // if Default is derived
        };

        // Act
        let result = subject.<method>(<args>);

        // Assert
        assert_eq!(result, <expected>);
    }

    // ── Panic / should_panic ───────────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "<message fragment>")]
    fn test_<function>_panics_when_<condition>() {
        <function>(<input_that_panics>);
    }

    // ── Async function (tokio runtime) ────────────────────────────────────────

    #[tokio::test]
    async fn test_<function>_<scenario>() {
        // Arrange
        let input = <value>;

        // Act
        let result = <async_function>(input).await;

        // Assert
        assert_eq!(result, <expected>);
    }

    // ── Async Result<T, E> ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_<function>_returns_ok_when_<condition>() {
        let result = <async_function>(<valid_input>).await;
        assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    }

    #[tokio::test]
    async fn test_<function>_returns_err_when_<condition>() {
        let result = <async_function>(<invalid_input>).await;
        assert!(result.is_err());
    }

    // ── Parameterized test (table-driven) ─────────────────────────────────────

    #[test]
    fn test_<function>_table_driven() {
        let cases: &[(<InputType>, <ExpectedType>)] = &[
            (<input_1>, <expected_1>),
            (<input_2>, <expected_2>),
            (<input_3>, <expected_3>),
        ];
        for (input, expected) in cases {
            assert_eq!(<function>(input), *expected, "input={input:?}");
        }
    }
}
```

## `tokio::test` setup

Add to `Cargo.toml` under `[dev-dependencies]` if not already present:

```toml
tokio = { version = "1", features = ["full", "test-util"] }
```

Use `#[tokio::test]` for any `async fn` — never wrap async bodies in `tokio::runtime::Runtime::new().unwrap().block_on(...)` in tests.

## Table-driven tests

Use a slice of `(input, expected)` tuples when testing the same function over many inputs. Include `input={input:?}` in the `assert_eq!` message so failures identify which case broke.

## Naming convention

| Pattern | Example |
|---------|---------|
| `test_<fn>_<scenario>` | `test_parse_email_valid` |
| `test_<fn>_returns_<result>_when_<condition>` | `test_divide_returns_err_when_zero` |
| `test_<Type>_<method>_<scenario>` | `test_Config_from_env_missing_key` |

## Rules

- One assertion per test when possible — makes failures easier to diagnose
- Name the scenario from the caller's perspective, not the implementation
- Do not import external crates just for tests unless they are already in `[dev-dependencies]`
- If a mock is needed and none exists, add `// TODO: mock <Trait> to test <fn>` and skip the test
