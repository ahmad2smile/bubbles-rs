use crossterm::style::Color;

use super::dimension::Dimension;

#[derive(Clone)]
pub struct View {
    pub content: String,
    pub dimension: Dimension,
    pub background: Color,
    pub color: Color,
}

impl View {
    pub fn new(content: String, dimension: Dimension, color: Color, background: Color) -> Self {
        Self {
            content,
            color,
            dimension,
            background,
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            content: "".to_owned(),
            dimension: Dimension::default(),
            color: Color::Black,
            background: Color::Black,
        }
    }
}
