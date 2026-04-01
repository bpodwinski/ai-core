# Axum — Routing

## Basic Routes

```rust
let app = Router::new()
    .route("/users", get(list_users).post(create_user))
    .route("/users/{id}", get(get_user).put(update_user).delete(delete_user));
```

## Path Parameters

```rust
use axum::extract::Path;

async fn get_user(Path(id): Path<u64>) -> String {
    format!("User {id}")
}

// Multiple params
async fn get_post(Path((user_id, post_id)): Path<(u64, u64)>) -> String {
    format!("User {user_id}, Post {post_id}")
}
```

## Nesting & Merging

```rust
// Nest under a prefix
let api = Router::new()
    .route("/users", get(list_users))
    .route("/posts", get(list_posts));

let app = Router::new()
    .nest("/api/v1", api);
// Routes: /api/v1/users, /api/v1/posts

// Merge two routers
let app = router_a.merge(router_b);
```

## Shared State

```rust
#[derive(Clone)]
struct AppState {
    db: PgPool,
}

let app = Router::new()
    .route("/users", get(list_users))
    .with_state(AppState { db: pool });

async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = sqlx::query("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();
    Json(users)
}
```

## Fallback & 404

```rust
let app = Router::new()
    .route("/", get(root))
    .fallback(|| async { (StatusCode::NOT_FOUND, "Not found") });
```
