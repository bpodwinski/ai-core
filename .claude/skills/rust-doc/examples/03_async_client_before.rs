// Example 03 — BEFORE: async trait + impl, no docs.

use std::time::Duration;

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("unexpected status {status}: {body}")]
    Status { status: u16, body: String },
    #[error("timeout after {0:?}")]
    Timeout(Duration),
}

pub trait ApiClient: Send + Sync {
    async fn get(&self, path: &str) -> ClientResult<String>;
    async fn post(&self, path: &str, body: &str) -> ClientResult<String>;
}

pub struct HttpClient {
    base_url: String,
    timeout: Duration,
    inner: reqwest::Client,
}

impl HttpClient {
    pub fn new(base_url: impl Into<String>, timeout: Duration) -> Self {
        Self {
            base_url: base_url.into(),
            timeout,
            inner: reqwest::Client::new(),
        }
    }
}

impl ApiClient for HttpClient {
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
