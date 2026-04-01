# Axum — Getting Started

Axum is a web framework built on Tokio, Tower, and Hyper.

## Setup

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.5", features = ["cors", "trace"] }
```

## Minimal Server

```rust
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Key Concepts

- **Router**: Tree of routes, composable via `.merge()` and `.nest()`
- **Handlers**: Async functions that take extractors and return responses
- **Extractors**: Types implementing `FromRequest` or `FromRequestParts`
- **State**: Shared app state via `State<T>` extractor
- **Middleware**: Tower layers and services
- **Error handling**: Via `IntoResponse` and custom error types
