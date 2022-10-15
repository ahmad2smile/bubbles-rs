## Progress bar

Code ([progress_bar.rs](./progress_bar.rs)): 

```rust
Progress::new(
    0,
    100,
    0,
    0,
    vec!["#5A56E0", "#EE6FF8"],
    Ok(&update_handler),
)
```
