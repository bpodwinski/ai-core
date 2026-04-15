// Example 01 — BEFORE: struct + fields + constructor + methods, nothing documented.
// This is the input the skill receives.

pub struct Config {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub timeout_ms: u64,
    pub tls: bool,
}

impl Config {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            max_connections: 10,
            timeout_ms: 5_000,
            tls: false,
        }
    }

    pub fn with_tls(mut self) -> Self {
        self.tls = true;
        self
    }

    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
