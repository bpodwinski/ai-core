// Example 01 — AFTER: same code, rustdoc added for every public item.
// No behavior change, no reformatting, only /// lines added.

/// Connection parameters for the remote service.
///
/// Build with [`Config::new`] then chain builder methods before passing to the client.
///
/// # Examples
///
/// ```
/// use my_crate::Config;
///
/// let cfg = Config::new("db.example.com", 5432)
///     .with_tls()
///     .with_timeout(3_000);
/// ```
pub struct Config {
    /// Hostname or IP address of the remote service.
    pub host: String,

    /// TCP port the service listens on.
    pub port: u16,

    /// Maximum number of concurrent connections held in the pool.
    pub max_connections: usize,

    /// Connection and request timeout in milliseconds.
    pub timeout_ms: u64,

    /// Whether to require TLS for every connection.
    pub tls: bool,
}

impl Config {
    /// Create a new `Config` with the given host and port.
    ///
    /// Defaults: `max_connections = 10`, `timeout_ms = 5_000`, `tls = false`.
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            max_connections: 10,
            timeout_ms: 5_000,
            tls: false,
        }
    }

    /// Enable TLS on all connections opened with this configuration.
    pub fn with_tls(mut self) -> Self {
        self.tls = true;
        self
    }

    /// Override the default timeout.
    ///
    /// `ms` is the number of milliseconds before a connection or request is aborted.
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    /// Return the `host:port` address string for use in connection URLs.
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
