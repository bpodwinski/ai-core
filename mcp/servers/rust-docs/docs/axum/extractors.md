# Axum — Extractors

Extractors pull data from HTTP requests. Order matters: body-consuming extractors must be last.

## Common Extractors

```rust
use axum::extract::{Path, Query, State, Json};
use axum::http::HeaderMap;

// Path parameters
async fn handler(Path(id): Path<u64>) -> String { .. }

// Query string: /search?q=hello&limit=10
#[derive(Deserialize)]
struct SearchParams { q: String, limit: Option<u32> }
async fn search(Query(params): Query<SearchParams>) -> String { .. }

// JSON body
#[derive(Deserialize)]
struct CreateUser { name: String, email: String }
async fn create(Json(body): Json<CreateUser>) -> impl IntoResponse { .. }

// Headers
async fn handler(headers: HeaderMap) -> String { .. }

// App state
async fn handler(State(db): State<PgPool>) -> String { .. }
```

## Multiple Extractors

```rust
async fn create_user(
    State(db): State<PgPool>,
    headers: HeaderMap,
    Json(body): Json<CreateUser>,  // body extractor LAST
) -> impl IntoResponse {
    // ...
}
```

## Custom Extractors

```rust
#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = parts.headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::Missing)?;
        
        validate_token(token).map_err(|_| AuthError::Invalid)
    }
}
```

## Responses

```rust
use axum::response::{IntoResponse, Json, Html};
use axum::http::StatusCode;

// String → text/plain
async fn text() -> &'static str { "Hello" }

// Json
async fn json() -> Json<Value> { Json(json!({"ok": true})) }

// Status + body
async fn created() -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(user))
}

// Html
async fn page() -> Html<&'static str> {
    Html("<h1>Hello</h1>")
}
```
