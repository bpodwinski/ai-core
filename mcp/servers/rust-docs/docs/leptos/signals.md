# Leptos — Signals & Reactivity

## Creating Signals

```rust
// Read/write pair
let (count, set_count) = signal(0);

// Single read-write signal
let count = RwSignal::new(0);
```

## Reading & Writing

```rust
// Read
let value = count.get();           // clones the value
let value = count.with(|v| *v);    // borrows without cloning

// Write
set_count.set(5);                  // replace value
set_count.update(|n| *n += 1);    // mutate in place
```

## Derived Signals

```rust
// Derived signal (recomputes when dependencies change)
let double = move || count.get() * 2;

// Memo (cached, only recomputes when value changes)
let double = Memo::new(move |_| count.get() * 2);
```

## Effects

```rust
// Runs whenever dependencies change
Effect::new(move |_| {
    log::info!("Count is: {}", count.get());
});
```

## Resources (Async)

```rust
// Async data loading
let data = Resource::new(
    move || user_id.get(),
    |id| async move { fetch_user(id).await },
);

view! {
    <Suspense fallback=|| view! { <p>"Loading..."</p> }>
        {move || data.get().map(|user| view! { <p>{user.name.clone()}</p> })}
    </Suspense>
}
```

## Rules

- Signals are `Copy` — no need to clone
- Always read signals inside reactive contexts (views, effects, memos)
- Use `Memo` when derived computation is expensive
- Use `Resource` for async data fetching
