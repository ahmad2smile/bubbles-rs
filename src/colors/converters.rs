use crossterm::style::Color;
use fast_srgb8::f32_to_srgb8;
use palette::{
    encoding::{Linear, Srgb},
    rgb::Rgb,
};

pub struct TypeAlias1(Rgb<Linear<Srgb>>);

impl From<Rgb<Linear<Srgb>>> for TypeAlias1 {
    fn from(val: Rgb<Linear<Srgb>>) -> Self {
        Self(val)
    }
}

impl From<(f32, f32, f32)> for TypeAlias1 {
    fn from(val: (f32, f32, f32)) -> Self {
        Self(Rgb::from(val))
    }
}

impl From<TypeAlias1> for Color {
    fn from(val: TypeAlias1) -> Self {
        let TypeAlias1(color) = val;

        Self::Rgb {
            r: f32_to_srgb8(color.red),
            g: f32_to_srgb8(color.green),
            b: f32_to_srgb8(color.blue),
        }
    }
}
