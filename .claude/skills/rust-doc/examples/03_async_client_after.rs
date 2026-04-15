// Example 03 — AFTER: async trait + impl, fully documented.
// Key points: trait contract in # Contract, # Errors on every async fn,
// struct field docs, and a doc-test on the constructor.

use std::time::Duration;

/// Convenience alias for results returned by API client methods.
pub type ClientResult<T> = Result<T, ClientError>;

/// Errors produced by HTTP API client operations.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// The underlying HTTP request failed (network error, DNS failure, etc.).
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// The server responded with a 4xx or 5xx status code.
    #[error("unexpected status {status}: {body}")]
    Status {
        /// HTTP status code returned by the server.
        status: u16,
        /// Response body included for debugging context.
        body: String,
    },

    /// The request did not complete within the configured timeout.
    #[error("timeout after {0:?}")]
    Timeout(Duration),
}

/// Asynchronous HTTP client abstraction over a base URL.
///
/// # Contract
///
/// Implementors must be `Send + Sync` and treat `path` as a relative URI
/// appended verbatim to the base URL. Status codes ≥ 400 must be returned as
/// [`ClientError::Status`], not silently swallowed.
pub trait ApiClient: Send + Sync {
    /// Send a GET request to `path` and return the response body as a string.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Request`] on network failure.
    /// Returns [`ClientError::Status`] if the server replies with a 4xx/5xx code.
    async fn get(&self, path: &str) -> ClientResult<String>;

    /// Send a POST request to `path` with the given `body` and return the
    /// response body as a string.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Request`] on network failure.
    /// Returns [`ClientError::Status`] if the server replies with a 4xx/5xx code.
    async fn post(&self, path: &str, body: &str) -> ClientResult<String>;
}

/// Concrete [`ApiClient`] backed by [`reqwest`].
pub struct HttpClient {
    /// Base URL prepended to every request path (e.g. `"https://api.example.com"`).
    base_url: String,

    /// Maximum duration allowed for a single request before [`ClientError::Timeout`] is returned.
    timeout: Duration,

    /// Underlying reqwest client (connection pool is shared across calls).
    inner: reqwest::Client,
}

impl HttpClient {
    /// Create a new `HttpClient` targeting `base_url` with the given per-request `timeout`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use my_crate::HttpClient;
    ///
    /// let client = HttpClient::new("https://api.example.com", Duration::from_secs(10));
    /// ```
    pub fn new(base_url: impl Into<String>, timeout: Duration) -> Self {
        Self {
            base_url: base_url.into(),
            timeout,
            inner: reqwest::Client::new(),
        }
    }
}

impl ApiClient for HttpClient {
    /// Send a GET request to `path` and return the response body as a string.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Request`] on network failure.
    /// Returns [`ClientError::Status`] if the server replies with a 4xx/5xx code.
    async fn get(&self, path: &str) -> ClientResult<String> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.inner.get(&url).timeout(self.timeout).send().await?;
        let status = resp.status().as_u16();
        let text = resp.text().await?;
        if status >= 400 {
            return Err(ClientError::Status { status, body: text });
        }
        Ok(text)
    }

    /// Send a POST request to `path` with the given `body` and return the
    /// response body as a string.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Request`] on network failure.
    /// Returns [`ClientError::Status`] if the server replies with a 4xx/5xx code.
    async fn post(&self, path: &str, body: &str) -> ClientResult<String> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .inner
            .post(&url)
            .body(body.to_string())
            .timeout(self.timeout)
            .send()
            .await?;
        let status = resp.status().as_u16();
        let text = resp.text().await?;
        if status >= 400 {
            return Err(ClientError::Status { status, body: text });
        }
        Ok(text)
    }
}
