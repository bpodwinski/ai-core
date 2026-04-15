# Rustdoc Templates

Reference templates for the most common Rust item kinds.
Copy the relevant block and adapt to context. One `///` block per item — no essays.

---

## Module

```rust
/// Provides <short description of what this module groups>.
///
/// # Overview
///
/// <1–2 sentences on the main types or responsibilities in this module.>
pub mod my_module { ... }
```

---

## Struct

```rust
/// <One-line summary — what does this struct represent or hold?>
///
/// <Optional: 1–2 sentence explanation of usage or lifecycle.>
///
/// # Examples
///
/// ```
/// use my_crate::MyStruct;
///
/// let s = MyStruct::new(<args>);
/// ```
pub struct MyStruct {
    /// <What this field holds and when it matters.>
    pub field_one: Type,

    /// <What this field controls or represents.>
    pub field_two: Type,
}
```

---

## Enum

```rust
/// <One-line: what does this enum represent or classify?>
#[derive(Debug)]
pub enum MyEnum {
    /// <What this variant means or when it is produced.>
    VariantOne,

    /// <What value this variant carries and what it means.>
    VariantTwo(Type),

    /// <Why this variant exists; distinct from the others.>
    VariantThree { field: Type },
}
```

---

## Trait

```rust
/// <One-line: what capability does implementing this trait provide?>
///
/// # Contract
///
/// Implementors must ensure <invariant, if any>.
pub trait MyTrait {
    /// <What this method does and what it returns.>
    fn required_method(&self) -> ReturnType;

    /// <What the default implementation does; when to override.>
    fn optional_method(&self) { ... }
}
```

---

## Free function

```rust
/// <Imperative verb phrase — what does this function do and return?>
///
/// Returns `None` / `Err(...)` when <condition>.
///
/// # Examples
///
/// ```
/// use my_crate::my_fn;
///
/// let result = my_fn(<input>);
/// assert_eq!(result, <expected>);
/// ```
pub fn my_fn(param: ParamType) -> ReturnType { ... }
```

---

## Constructor (`new` / `from_*`)

```rust
/// Create a new [`MyStruct`] from <description of inputs>.
///
/// # Panics
///
/// Panics if <condition> — e.g. `capacity` is zero.
pub fn new(param: ParamType) -> Self { ... }
```

---

## Fallible function (`Result`)

```rust
/// <What this function does.>
///
/// # Errors
///
/// Returns [`ErrorKind::NotFound`] if <condition>.
/// Returns [`ErrorKind::InvalidInput`] if <condition>.
pub fn fallible_fn(param: ParamType) -> Result<Output, MyError> { ... }
```

---

## Async function

```rust
/// <Imperative phrase — what does this async function accomplish?>
///
/// # Errors
///
/// Returns an error if <condition, e.g. network failure, timeout>.
pub async fn async_fn(param: ParamType) -> Result<Output, MyError> { ... }
```

---

## Error type

```rust
/// Errors that can occur in <this module / crate subsystem>.
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    /// The requested resource was not found at `<path>`.
    #[error("not found: {0}")]
    NotFound(String),

    /// An I/O error occurred while <reading / writing / ...>.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
```

---

## Type alias

```rust
/// Shorthand for <what the underlying type represents in this domain>.
pub type MyAlias = HashMap<String, Vec<u64>>;
```

---

## Constant / static

```rust
/// Maximum number of retry attempts before returning an error.
pub const MAX_RETRIES: usize = 3;
```

---

## Naming checklist

| Item | First word |
|------|-----------|
| `fn` | Verb: "Return", "Build", "Load", "Parse", "Validate", "Emit" |
| `struct` / `enum` | Noun phrase: "Represents …", "Holds …", "Describes …" |
| `trait` | Capability: "Provides …", "Enables …", "Abstracts …" |
| `mod` | Noun group: "Provides …", "Contains …", "Groups …" |
| Field/variant | Noun or noun phrase |

## Common mistakes to avoid

- `/// This function does X` → drop "This function", start with verb
- `/// A struct that holds Y` → drop "A struct that", start with noun
- Documenting `pub(crate)` items (skip unless asked)
- Restating the type signature in prose
- Multi-paragraph essays on simple items
- Guessing behavior when context is unclear — keep it generic
