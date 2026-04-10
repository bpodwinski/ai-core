# Example — Result<T, E>

Functions that can fail. Tests cover the success path, each distinct error variant, and boundary inputs.

## Source (before)

```rust
// src/parser.rs

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Empty,
    InvalidInt(ParseIntError),
    OutOfRange(i64),
}

pub fn parse_port(s: &str) -> Result<u16, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    let n: i64 = s.trim().parse().map_err(ParseError::InvalidInt)?;
    if !(1..=65535).contains(&n) {
        return Err(ParseError::OutOfRange(n));
    }
    Ok(n as u16)
}
```

## Source (after — with tests added)

```rust
// src/parser.rs

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Empty,
    InvalidInt(ParseIntError),
    OutOfRange(i64),
}

pub fn parse_port(s: &str) -> Result<u16, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    let n: i64 = s.trim().parse().map_err(ParseError::InvalidInt)?;
    if !(1..=65535).contains(&n) {
        return Err(ParseError::OutOfRange(n));
    }
    Ok(n as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_port_valid() {
        assert_eq!(parse_port("8080"), Ok(8080));
    }

    #[test]
    fn test_parse_port_valid_with_whitespace() {
        assert_eq!(parse_port("  443  "), Ok(443));
    }

    #[test]
    fn test_parse_port_min_boundary() {
        assert_eq!(parse_port("1"), Ok(1));
    }

    #[test]
    fn test_parse_port_max_boundary() {
        assert_eq!(parse_port("65535"), Ok(65535));
    }

    #[test]
    fn test_parse_port_returns_err_when_empty() {
        assert!(matches!(parse_port(""), Err(ParseError::Empty)));
    }

    #[test]
    fn test_parse_port_returns_err_when_not_a_number() {
        assert!(matches!(parse_port("abc"), Err(ParseError::InvalidInt(_))));
    }

    #[test]
    fn test_parse_port_returns_err_when_zero() {
        assert!(matches!(parse_port("0"), Err(ParseError::OutOfRange(0))));
    }

    #[test]
    fn test_parse_port_returns_err_when_above_max() {
        assert!(matches!(parse_port("65536"), Err(ParseError::OutOfRange(65536))));
    }

    #[test]
    fn test_parse_port_returns_err_when_negative() {
        assert!(matches!(parse_port("-1"), Err(ParseError::OutOfRange(-1))));
    }
}
```
