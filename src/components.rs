use crossterm::style::Color;

pub mod progress;

pub struct Instruction {
    pub content: String,
    pub x: u16,
    pub y: u16,
    pub background: Color,
    pub color: Color,
}

impl Instruction {
    pub fn new(content: String, color: Color, x: u16, y: u16, background: Color) -> Self {
        Self {
            content,
            color,
            x,
            y,
            background,
        }
    }
}

pub trait Component {
    fn render(&self, instructions: &mut Vec<Instruction>);
}
