use crossterm::style::Color;

#[derive(Clone)]
pub struct View {
    pub content: String,
    pub x: u16,
    pub y: u16,
    pub background: Color,
    pub color: Color,
}

impl View {
    pub fn new(content: String, x: u16, y: u16, color: Color, background: Color) -> Self {
        Self {
            content,
            color,
            x,
            y,
            background,
        }
    }

    pub fn default() -> Self {
        Self {
            content: "".to_owned(),
            x: 0,
            y: 0,
            color: Color::Black,
            background: Color::Black,
        }
    }
}
