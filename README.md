# `yeet!`

Replace `return Err(x)` and `return None` with:

```rust
fn foo() -> Option<u32> {
    yeet!();
}

fn bar() -> Result<u32, u32> {
    yeet!(42);
}

fn baz(x: u32) -> Result<u32, u32> {
    if x % 2 == 1 {
        yeet!(x);
    }
    Ok(x)
}
```

That's pretty much it.
