# Bubbles

## Intro

TUI Framework for Rust inspired by [bubbles](https://github.com/charmbracelet/bubbles) from Go.

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
![image](https://user-images.githubusercontent.com/6108922/196053815-7e8b1e87-4546-4e41-9343-229d91007123.png)


Further examples under [bin](./src/bin/examples/)