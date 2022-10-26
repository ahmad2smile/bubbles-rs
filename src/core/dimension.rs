#[derive(Clone, Copy)]
pub struct Dimension {
    pub x: u16,
    pub y: u16,
    pub width: usize,
    pub height: usize,
}

impl Dimension {
    pub fn new(x: u16, y: u16, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}
