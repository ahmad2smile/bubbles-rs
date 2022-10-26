use bubbles::{
    components::input::Input,
    core::{
        dimension::Dimension,
        renderer::{OnKeyPress, Renderer},
        style::Style,
    },
};
use crossterm::{
    event::{KeyCode, KeyEvent},
    style::Color,
};

pub fn update_handler(p: &mut Input, event: KeyEvent) {
    match event.code {
        KeyCode::Backspace => {
            p.value.pop();
        }
        KeyCode::Char(value) => p.value.push(value),
        KeyCode::Enter => todo!(),
        KeyCode::Left => todo!(),
        KeyCode::Right => todo!(),
        KeyCode::Up => todo!(),
        KeyCode::Down => todo!(),
        KeyCode::Home => todo!(),
        KeyCode::End => todo!(),
        KeyCode::PageUp => todo!(),
        KeyCode::PageDown => todo!(),
        KeyCode::Tab => todo!(),
        KeyCode::BackTab => todo!(),
        KeyCode::Delete => todo!(),
        KeyCode::Insert => todo!(),
        KeyCode::F(_) => todo!(),
        KeyCode::Null => todo!(),
        KeyCode::Esc => todo!(),
        KeyCode::CapsLock => todo!(),
        KeyCode::ScrollLock => todo!(),
        KeyCode::NumLock => todo!(),
        KeyCode::PrintScreen => todo!(),
        KeyCode::Pause => todo!(),
        KeyCode::Menu => todo!(),
        KeyCode::KeypadBegin => todo!(),
        KeyCode::Media(_) => todo!(),
        KeyCode::Modifier(_) => todo!(),
    }
}

pub fn create_input_field<'a>() -> Result<Input<'a>, ()> {
    let renderer = Renderer::from(OnKeyPress(&update_handler));

    let root = Input::new(
        "Initial placeholder".to_string(),
        Style::new(vec![], Color::White, Color::Black),
        Dimension::new(0, 4, 100, 0),
        renderer,
    );

    Ok(root)
}
