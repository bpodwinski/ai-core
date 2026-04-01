# Leptos — Getting Started

Leptos is a full-stack Rust framework for building reactive web applications.

## Setup

```bash
cargo install cargo-leptos
cargo leptos new --git leptos-rs/start-axum
cd my-project
cargo leptos watch
```

## Minimal App

```rust
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Clicked: " {count}
        </button>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
```

## Key Concepts

- **Signals**: Reactive state with `signal()`, `RwSignal::new()`
- **Effects**: Side effects with `Effect::new()`
- **Components**: Functions returning `impl IntoView`
- **Server Functions**: `#[server]` for server-side logic callable from client
- **SSR**: Built-in server-side rendering with hydration
