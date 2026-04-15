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
}
```

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
