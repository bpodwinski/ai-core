# Example — Pure Function

A deterministic function with no side effects. Tests focus on input/output pairs and edge cases.

## Source (before)

```rust
// src/math.rs

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn percentage(part: u64, total: u64) -> f64 {
    if total == 0 {
        return 0.0;
    }
    (part as f64 / total as f64) * 100.0
}
```

## Source (after — with tests added)

```rust
// src/math.rs

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn percentage(part: u64, total: u64) -> f64 {
    if total == 0 {
        return 0.0;
    }
    (part as f64 / total as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    // clamp

    #[test]
    fn test_clamp_returns_value_when_in_range() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
    }

    #[test]
    fn test_clamp_returns_min_when_below() {
        assert_eq!(clamp(-3.0, 0.0, 10.0), 0.0);
    }

    #[test]
    fn test_clamp_returns_max_when_above() {
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_clamp_returns_min_when_equal_to_min() {
        assert_eq!(clamp(0.0, 0.0, 10.0), 0.0);
    }

    // percentage

    #[test]
    fn test_percentage_nominal() {
        assert_eq!(percentage(50, 200), 25.0);
    }

    #[test]
    fn test_percentage_full() {
        assert_eq!(percentage(100, 100), 100.0);
    }

    #[test]
    fn test_percentage_zero_total_returns_zero() {
        assert_eq!(percentage(10, 0), 0.0);
    }

    #[test]
    fn test_percentage_zero_part() {
        assert_eq!(percentage(0, 100), 0.0);
    }
}
```
