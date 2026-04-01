# Rust — Async / Await

## Basics

```rust
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let data = fetch_data("https://example.com").await.unwrap();
}
```

## Concurrency

```rust
use tokio::join;

// Run concurrently (not in parallel — same thread)
let (users, posts) = join!(fetch_users(), fetch_posts());

// Spawn a background task (parallel)
let handle = tokio::spawn(async move {
    expensive_computation().await
});
let result = handle.await?;

// Select first to complete
tokio::select! {
    val = future_a => println!("a finished: {val:?}"),
    val = future_b => println!("b finished: {val:?}"),
}
```

## Streams

```rust
use tokio_stream::StreamExt;

let mut stream = tokio_stream::iter(vec![1, 2, 3]);
while let Some(value) = stream.next().await {
    println!("{value}");
}
```

## Channels

```rust
use tokio::sync::{mpsc, oneshot};

// Multi-producer, single-consumer
let (tx, mut rx) = mpsc::channel(32);
tokio::spawn(async move { tx.send("hello").await.unwrap(); });
let msg = rx.recv().await;

// One-shot (single value)
let (tx, rx) = oneshot::channel();
tx.send(42).unwrap();
let value = rx.await?;
```

## Common Patterns

- `tokio::time::sleep(Duration::from_secs(1)).await` — async sleep
- `tokio::time::timeout(dur, future).await` — timeout a future
- `tokio::sync::Mutex` — async-aware mutex (not `std::sync::Mutex` in async)
- `tokio::sync::RwLock` — async read-write lock
