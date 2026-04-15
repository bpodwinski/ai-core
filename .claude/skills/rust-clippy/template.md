# Clippy Lint Fix Patterns

Common lint categories with before/after examples.

---

## `clippy::unwrap_used` / `clippy::expect_used`

Replace `.unwrap()` / `.expect()` with `?` or a proper error message.

```rust
// Before
let val = map.get("key").unwrap();
let n: i32 = s.parse().expect("must be int");

// After — propagate with ?
let val = map.get("key").ok_or_else(|| anyhow!("missing key"))?;
let n: i32 = s.parse().context("expected integer")?;

// After — when inside a context that can't return Result, use unwrap_or_else
let val = map.get("key").unwrap_or_else(|| &default_val);
```

---

## `clippy::needless_pass_by_value`

Change owned parameter to a reference when the function doesn't consume it.

```rust
// Before (clippy warns: the function doesn't use ownership)
fn print_name(name: String) {
    println!("{name}");
}

// After
fn print_name(name: &str) {
    println!("{name}");
}
```

---

## `clippy::redundant_closure`

Replace unnecessary closures with direct function references.

```rust
// Before
let results: Vec<_> = items.iter().map(|x| x.to_string()).collect();
let parsed: Vec<_> = strs.iter().map(|s| s.parse::<i32>()).collect();

// After
let results: Vec<_> = items.iter().map(ToString::to_string).collect();
let parsed: Vec<_> = strs.iter().map(str::parse::<i32>).collect();
```

---

## `clippy::clone_on_ref_ptr` / `clippy::redundant_clone`

Remove clones that aren't needed.

```rust
// Before
fn process(s: &String) -> usize {
    let owned = s.clone();  // redundant if we only read it
    owned.len()
}

// After
fn process(s: &str) -> usize {
    s.len()
}
```

---

## `clippy::match_wildcard_for_single_variants`

Replace `_ =>` with the actual variant when only one case is unmatched.

```rust
// Before
match status {
    Status::Active => do_thing(),
    _ => {}
}

// After
match status {
    Status::Active => do_thing(),
    Status::Inactive | Status::Pending => {}
}
```

---

## `clippy::if_not_else` / `clippy::needless_bool`

Simplify boolean expressions.

```rust
// Before
if !condition {
    false
} else {
    true
}

// After
condition
```

---

## `clippy::range_plus_one`

Use inclusive ranges instead of `..n+1`.

```rust
// Before
for i in 0..n + 1 { ... }

// After
for i in 0..=n { ... }
```

---

## `clippy::manual_map`

Replace `match`/`if let` over `Option` with `.map()`.

```rust
// Before
let y = match x {
    Some(v) => Some(v * 2),
    None => None,
};

// After
let y = x.map(|v| v * 2);
```

---

## `clippy::format_collect`

Replace `.map(|s| format!(...)).collect::<String>()` with join or direct write.

```rust
// Before
let s: String = items.iter().map(|i| format!("{i}, ")).collect();

// After
let s = items.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ");
```

---

## Suppression pattern (false positives)

When a lint fires but the fix would degrade correctness or readability, suppress with a reason:

```rust
#[allow(clippy::too_many_arguments)] // all args required by the HTTP handler signature
fn handle_request(method: &str, path: &str, query: &str, body: &[u8], headers: &HeaderMap, state: Arc<AppState>, tx: Sender<Event>) { ... }
```
