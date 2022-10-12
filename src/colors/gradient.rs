use palette::{
    encoding::{Linear, Srgb},
    rgb::Rgb,
    Gradient, LinSrgb, Shade,
};
use std::str::FromStr;

pub struct Gradients {}

impl Gradients {
    pub fn from_color(colors: &[&str], resolution: usize) -> Vec<Rgb<Linear<Srgb>>> {
        let mut gradient_starts = Vec::new();
        let mut gradient_ends = Vec::new();

        let (start, end) = if colors.len() > 2 {
            colors.split_at(colors.len() / 2)
        } else {
            (&colors[..1], &colors[1..])
        };

        for color in start {
            let c = LinSrgb::from_str(color).unwrap().into_format();

            let mut gs = vec![
                c.darken_fixed(0.3),
                c.darken_fixed(0.3),
                c.darken_fixed(0.2),
            ];

            gradient_starts.append(&mut gs);
        }

        for color in end {
            let c = LinSrgb::from_str(color).unwrap().into_format();

            let mut gs = vec![
                c.darken_fixed(0.2),
                c.darken_fixed(0.3),
                c.darken_fixed(0.4),
            ];

            gradient_ends.append(&mut gs);
        }

        let gradient = Gradient::new(vec![gradient_starts, gradient_ends].concat());

        gradient.take(resolution).collect()
    }
}
