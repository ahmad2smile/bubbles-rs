use std::cmp;

use crossterm::style::{Color, Stylize};

use crate::{
    colors::gradient::Gradients,
    core::{render::Render, state::State, view::View},
};

/// Progress bar
pub struct Progress<'a> {
    pub value: usize,
    pub length: usize,
    pub x: u16,
    pub y: u16,
    pub colors: Vec<&'static str>,
    content: &'static str,
    content_empty: &'static str,
    pub(crate) update_handler: Result<&'a dyn Fn(&mut Self), ()>,
}

impl<'a> Progress<'a> {
    const CONTENT: &'static str = "█";
    const CONTENT_EMPTY: &'static str = "░";

    pub fn new(
        value: usize,
        length: usize,
        x: u16,
        y: u16,
        colors: Vec<&'static str>,
        update_handler: Result<&'a dyn Fn(&mut Self), ()>,
    ) -> Self {
        Self {
            value,
            length,
            x,
            y,
            colors,
            content: Progress::CONTENT,
            content_empty: Progress::CONTENT_EMPTY,
            update_handler: update_handler,
        }
    }

    pub fn default() -> Self {
        Self {
            value: 0,
            length: 100,
            x: 0,
            y: 0,
            colors: vec!["#AAAAAA", "#FFFFFF"],
            content: Progress::CONTENT,
            content_empty: Progress::CONTENT_EMPTY,
            update_handler: Err(()),
        }
    }

    pub fn update_value(&mut self, value: usize) {
        self.value = cmp::min(self.length, value);
    }
}

impl Render for Progress<'_> {
    fn render(&self) -> View {
        let colors = Gradients::from_colors(&self.colors, self.length);

        let content_len = cmp::min(self.value, self.length);

        let bar_content = colors[..content_len]
            .iter()
            .map(|c| self.content.to_owned().with(c.to_owned()).to_string())
            .collect::<Vec<String>>()
            .join("")
            + &self.content_empty.repeat(self.length - self.value);

        let current_content = bar_content
            + format!(" {}%", self.value)
                .with(Color::DarkGrey)
                .to_string()
                .as_str();

        View {
            content: current_content,
            x: self.x,
            y: self.y,
            background: Color::Reset,
            color: Color::Grey,
        }
    }
}

impl<'a> State<'a> for Progress<'a> {
    fn on_update(&mut self, handler: &'a dyn Fn(&mut Self)) {
        self.update_handler = Ok(handler);
    }

    fn handle_update(&mut self) {
        let handler = &self.update_handler.unwrap();

        handler(self);
    }
}
