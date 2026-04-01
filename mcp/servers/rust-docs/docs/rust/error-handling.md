# Rust — Error Handling

## Result & Option

```rust
// Result<T, E> — recoverable errors
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

// Option<T> — absence of value
fn find_user(id: u64) -> Option<User> {
    users.iter().find(|u| u.id == id).cloned()
}
```

## The ? Operator

```rust
fn read_config() -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string("config.toml")?;  // propagates error
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

## Custom Error Types

```rust
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

// For Axum integration
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".into()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}
```

## anyhow vs thiserror

- **`thiserror`**: Define specific error types (libraries, public APIs)
- **`anyhow`**: Catch-all error handling (applications, scripts)

```rust
// thiserror — structured errors
#[derive(Debug, thiserror::Error)]
enum MyError { #[error("...")] Variant(#[from] OtherError) }

// anyhow — quick and flexible
fn main() -> anyhow::Result<()> {
    let f = File::open("data.json").context("failed to open config")?;
    Ok(())
}
```
