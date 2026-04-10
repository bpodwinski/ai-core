# Example — Struct Methods

Methods on a struct. Build the minimal instance needed — avoid constructing unrelated fields when `Default` is available.

## Source (before)

```rust
// src/config.rs

#[derive(Debug, Default)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
}

impl Config {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            max_connections: 100,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn is_valid(&self) -> bool {
        !self.host.is_empty() && self.port > 0
    }

    pub fn with_max_connections(mut self, n: usize) -> Self {
        self.max_connections = n;
        self
    }
}
```

## Source (after — with tests added)

```rust
// src/config.rs

#[derive(Debug, Default)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
}

impl Config {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            max_connections: 100,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn is_valid(&self) -> bool {
        !self.host.is_empty() && self.port > 0
    }

    pub fn with_max_connections(mut self, n: usize) -> Self {
        self.max_connections = n;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config() -> Config {
        Config::new("localhost", 8080)
    }

    // Config::new

    #[test]
    fn test_Config_new_sets_host_and_port() {
        let cfg = Config::new("example.com", 443);
        assert_eq!(cfg.host, "example.com");
        assert_eq!(cfg.port, 443);
    }

    #[test]
    fn test_Config_new_sets_default_max_connections() {
        let cfg = Config::new("localhost", 80);
        assert_eq!(cfg.max_connections, 100);
    }

    // Config::address

    #[test]
    fn test_Config_address_formats_correctly() {
        assert_eq!(make_config().address(), "localhost:8080");
    }

    // Config::is_valid

    #[test]
    fn test_Config_is_valid_returns_true_when_host_and_port_set() {
        assert!(make_config().is_valid());
    }

    #[test]
    fn test_Config_is_valid_returns_false_when_host_empty() {
        let cfg = Config { host: String::new(), port: 80, ..Default::default() };
        assert!(!cfg.is_valid());
    }

    #[test]
    fn test_Config_is_valid_returns_false_when_port_zero() {
        let cfg = Config { host: "localhost".into(), port: 0, ..Default::default() };
        assert!(!cfg.is_valid());
    }

    // Config::with_max_connections

    #[test]
    fn test_Config_with_max_connections_overrides_default() {
        let cfg = make_config().with_max_connections(50);
        assert_eq!(cfg.max_connections, 50);
    }

    #[test]
    fn test_Config_with_max_connections_allows_zero() {
        let cfg = make_config().with_max_connections(0);
        assert_eq!(cfg.max_connections, 0);
    }
}
```
