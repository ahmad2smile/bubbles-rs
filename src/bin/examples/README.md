## Progress bar

Code ([progress_bar.rs](./progress_bar.rs)): 

```rust
Progress::new(
    0,
    Style::new(gradient, Color::Grey, Color::Reset),
    Dimension::new(0, 2, 100, 0),
    OnRender::new(&update_handler),
);
```

![image](https://user-images.githubusercontent.com/6108922/196055972-9af622ae-d5cc-48fb-b7a5-504a954ec9bf.png)
