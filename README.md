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
![image](https://user-images.githubusercontent.com/6108922/196055972-9af622ae-d5cc-48fb-b7a5-504a954ec9bf.png)


Further examples under [bin](./src/bin/examples/)