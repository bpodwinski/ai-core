# Leptos — Components

## Defining Components

```rust
#[component]
fn MyComponent(
    /// Required prop
    name: String,
    /// Optional prop with default
    #[prop(optional)] class: &'static str,
    /// Optional prop with default value
    #[prop(default = 42)] count: i32,
    /// Children
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class>
            <h1>{name}</h1>
            <p>"Count: " {count}</p>
            {children()}
        </div>
    }
}
```

## Using Components

```rust
view! {
    <MyComponent name="Hello".to_string()>
        <p>"Child content"</p>
    </MyComponent>
}
```

## Dynamic Rendering

```rust
// Conditional
view! {
    <Show when=move || count.get() > 5 fallback=|| view! { <p>"Too low"</p> }>
        <p>"High enough!"</p>
    </Show>
}

// Lists
view! {
    <For each=move || items.get() key=|item| item.id let:item>
        <li>{item.name.clone()}</li>
    </For>
}
```

## Component Patterns

- Use `#[component]` on any function returning `impl IntoView`
- Props are passed as function parameters
- `Children` type for slot content
- `#[prop(into)]` for automatic type conversion
- `#[prop(optional)]` for optional props (defaults to `Default::default()`)
