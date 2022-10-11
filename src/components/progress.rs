use std::str::FromStr;

use crossterm::style::Color;
use fast_srgb8::f32_to_srgb8;
use palette::{Gradient, LinSrgb, Shade};

use super::{Component, Instruction};

pub struct Progress {
    content: &'static str,
    pub length: u32,
}

impl Progress {
    const CONTENT: &'static str = "█";

    pub fn new(length: u32) -> Self {
        Self {
            length,
            content: Progress::CONTENT,
        }
    }

    pub fn default() -> Self {
        Self {
            length: 100,
            content: Progress::CONTENT,
        }
    }
}

impl Component for Progress {
    fn render(&self, instructions: &mut Vec<Instruction>) {
        let color_a = LinSrgb::from_str("#5A56E0").unwrap().into_format();
        let color_b = LinSrgb::from_str("#EE6FF8").unwrap().into_format();

        let gradient = Gradient::new(vec![
            color_a.darken_fixed(0.3),
            color_a.darken_fixed(0.3),
            color_a.darken_fixed(0.2),
            color_b.darken_fixed(0.2),
            color_b.darken_fixed(0.3),
            color_b.darken_fixed(0.4),
        ]);

        let factor: usize = 100;
        let resolution: usize = self.length as usize * factor;

        let colors: Vec<_> = gradient.take(resolution).collect();

        for x in 0..resolution {
            let c = colors[x];
            // let c1 = Srgb::from_linear(c);

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
