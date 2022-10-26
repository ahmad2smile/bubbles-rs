use crossterm::style::Color;

#[derive(Clone)]
pub struct Style {
    pub gradient: Vec<&'static str>,
    pub color: Color,
    pub background: Color,
}

impl Style {
    pub fn new(gradient: Vec<&'static str>, color: Color, background: Color) -> Self {
        Self {
            gradient,
            color,
            background,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            gradient: vec!["#AAAAAA", "#FFFFFF"],
            color: Color::DarkGrey,
            background: Color::Reset,
        }
    }
}
