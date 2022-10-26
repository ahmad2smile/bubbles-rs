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


Code ([input_field.rs](./input_field.rs)): 

```rust
Input::new(
    "Initial placeholder".to_string(),
    Style::new(vec![], Color::White, Color::Black),
    Dimension::new(0, 4, 100, 0),
    renderer,
);
```

![image](https://user-images.githubusercontent.com/6108922/198144900-4582f5b7-4e16-4274-92e8-ab816910e2fe.png)