use crossterm::style::Color;
use fast_srgb8::f32_to_srgb8;

use crate::colors::gradient::Gradients;

use super::{Component, Instruction};

pub struct Progress {
    content: &'static str,
    pub length: u32,
    pub colors: Vec<&'static str>,
}

impl Progress {
    const CONTENT: &'static str = "█";

    pub fn new(length: u32, colors: Vec<&'static str>) -> Self {
        Self {
            length,
            content: Progress::CONTENT,
            colors,
        }
    }

    pub fn default() -> Self {
        Self {
            length: 100,
            content: Progress::CONTENT,
            colors: vec!["#AAAAAA", "#FFFFFF"],
        }
    }
}

impl Component for Progress {
    fn render(&self, instructions: &mut Vec<Instruction>) {
        let factor: usize = 100;
        let resolution: usize = self.length as usize * factor;

        let colors: Vec<_> = Gradients::from_color(&self.colors, resolution);

        for x in 0..resolution {
            let c = colors[x];

            let color = Color::Rgb {
                r: f32_to_srgb8(c.red),
                g: f32_to_srgb8(c.green),
                b: f32_to_srgb8(c.blue),
            };

            instructions.push(Instruction::new(
                self.content.to_string(),
                color,
                (x / factor).try_into().unwrap(),
                0,
                color,
            ));
        }
    }
}
